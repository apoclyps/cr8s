use diesel::{Connection, PgConnection};

use crate::{
    auth,
    models::{NewUser, Role, User, UserRole},
    repositories::{RoleRepository, UserRepository},
};

fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot load DB url from env");
    PgConnection::establish(&database_url).expect("Cannot connect to Posgres DB")
}

pub fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let c = load_db_connection();

    let password_hash: String = auth::hash_password(password).unwrap();
    let new_user: NewUser = NewUser {
        username: username,
        password: password_hash,
    };
    let user: User = UserRepository::create(&c, new_user, role_codes).unwrap();

    println!("User created {:?}", user);

    let roles: Vec<Role> = RoleRepository::find_by_user(&c, &user).unwrap();

    println!("Roles assigned {:?}", roles);
}

pub fn list_users() {
    let c: PgConnection = load_db_connection();
    let users: Vec<(User, Vec<(UserRole, Role)>)> = UserRepository::find_with_roles(&c).unwrap();
    for user in users {
        println!("User: {:?}", user);
    }
}

pub fn delete_user(id: i32) {
    let c = load_db_connection();

    UserRepository::delete(&c, id).unwrap();
}
