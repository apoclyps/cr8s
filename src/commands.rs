use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use diesel::{Connection, PgConnection};

use crate::{
    models::NewUser,
    repositories::{RoleRepository, UserRepository},
};

fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot load DB url from env");
    PgConnection::establish(&database_url).expect("Cannot connect to Posgres DB")
}

fn hash_password(password: String) -> String {
    let salt = SaltString::generate(OsRng);
    let argon = argon2::Argon2::default();
    let password_hash = argon.hash_password(password.as_bytes(), &salt).unwrap();

    password_hash.to_string()
}

pub fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let c = load_db_connection();

    let new_user = NewUser {
        username: username,
        password: hash_password(password),
    };
    let user = UserRepository::create(&c, new_user, role_codes).unwrap();

    println!("User created {:?}", user);

    let roles = RoleRepository::find_by_user(&c, &user).unwrap();

    println!("Roles assigned {:?}", roles);
}

pub fn list_users() {
    let c: PgConnection = load_db_connection();

    let users = UserRepository::find_with_roles(&c).unwrap();
    for user in users {
        println!("User: {:?}", user);
    }
}

pub fn delete_user(id: i32) {
    let c = load_db_connection();

    UserRepository::delete(&c, id).unwrap();
}
