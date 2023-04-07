use std::error::Error;

use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::{json, Value};

pub mod authorization;
pub mod crates;
pub mod rustaceans;

use diesel::PgConnection;
use rocket_sync_db_pools::database;

#[database("postgres")]
pub struct DbConn(PgConnection);

fn server_error(error: &Box<dyn Error>) -> Custom<Value> {
    log::error!("{}", error);
    Custom(Status::InternalServerError, json!("Something went wrong"))
}
