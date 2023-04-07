pub mod authorization;
pub mod rustaceans;
pub mod crates;

use std::error::Error;

use diesel::PgConnection;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::serde_json::{json, Value};
use rocket::response::status::{Custom};
use rocket::http::Status;
use rocket_sync_db_pools::database;
use rocket_db_pools::{Database, deadpool_redis, Connection, deadpool_redis::redis::AsyncCommands};

use crate::models::{User, RoleCode};
use crate::repositories::{UserRepository, RoleRepository};

#[database("postgres")]
pub struct DbConn(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Pool);

fn server_error(error: &Box<dyn Error>) -> Custom<Value> {
    log::error!("{}", error);
    Custom(Status::InternalServerError, json!("Something went wrong"))
}

pub struct EditorUser(User);


#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditorUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = request.guard::<User>().await
            .expect("Cannot retrieve logged in user in request guard");
        let db = request.guard::<DbConn>().await
            .expect("Cannot connect to postgres in request guard");

        let has_access = db.run(|c| {
            match RoleRepository::find_by_user(c, &user) {
                Ok(roles) => {
                    log::info!("Got request with {:?}", roles);
                    let accessible = roles.into_iter().any(|r| match r.code {
                        RoleCode::Admin => true,
                        RoleCode::Editor => true,
                        _ => false
                    });
                    log::info!("Accessible is {:?}", accessible);
                    accessible.then_some(EditorUser(user))
                },
                Err(_) => None,
            }
        }).await;

        match has_access {
            Some(eu) => Outcome::Success(eu),
            _ => Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Authorization: Bearer SESSION_ID_128_CHARS_LONG
        let session_header = request.headers().get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        if let Some(header_value) = session_header {
            let mut cache = request.guard::<Connection<CacheConn>>().await
                .expect("Cannot connect to redis in request guard");
            let db = request.guard::<DbConn>().await
                .expect("Cannot connect to postgres in request guard");
            let result = cache.get::<_, i32>(format!("sessions/{}", header_value[1])).await;
            if let Ok(user_id) = result {
                return match db.run(move |c| UserRepository::find(&c, user_id)).await {
                    Ok(user) => Outcome::Success(user),
                    _ => Outcome::Failure((Status::Unauthorized, ())),
                }
            }

        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}
