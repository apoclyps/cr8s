use chrono::{Datelike, Utc};
use diesel::{Connection, PgConnection};
use lettre::transport::smtp::authentication::Credentials;
use tera::{Context, Tera};

use crate::{
    auth,
    mail::HtmlMailer,
    models::{NewUser, RoleCode},
    repositories::{CrateRepository, RoleRepository, UserRepository},
};

fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot load DB url from env");
    PgConnection::establish(&database_url).expect("Cannot connect to Posgres DB")
}

pub fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let c = load_db_connection();

    let password_hash: String = auth::hash_password(password).unwrap();
    let new_user: NewUser = NewUser {
        username,
        password: password_hash,
    };
    let role_codes = role_codes
        .iter()
        .map(|v| RoleCode::from_string(v.to_owned()).unwrap())
        .collect();
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

fn load_template_engine() -> Tera {
    Tera::new("templates/**/*.html").unwrap_or_else(|e| {
        panic!("Parsing error(s): {}", e);
    })
}

pub fn send_digest(to: String, hours_since: i32) {
    let c = load_db_connection();
    let tera = load_template_engine();

    let crates = CrateRepository::find_since(&c, hours_since).unwrap();
    if crates.len() == 0 {
        println!("No crates found since {} hours", hours_since);
        return;
    }

    println!("Sending the digest for {} crates", crates.len());

    let year = Utc::now().year();
    let mut context = Context::new();
    context.insert("crates", &crates);
    context.insert("year", &year);

    let smtp_host = std::env::var("SMTP_HOST").expect("Cannot load SMTP host from env");
    let smtp_username = std::env::var("SMTP_USERNAME").expect("Cannot load SMTP username from env");
    let smtp_password = std::env::var("SMTP_PASSWORD").expect("Cannot load SMTP password from env");

    let credentials = Credentials::new(smtp_username, smtp_password);
    let mailer = HtmlMailer {
        smtp_host,
        credentials,
        template_engine: tera,
    };

    mailer.send(&to, "email/digest.html", &context).unwrap();
}
