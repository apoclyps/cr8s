use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{serde_json::json, Json, Value};

use super::{server_error, EditorUser};
use crate::diesel::result::Error::NotFound;
use crate::models::{NewRustacean, Rustacean};
use crate::repositories::RustaceanRepository;
use crate::rocket_routes::DbConn;

#[get("/rustaceans")]
pub async fn get_rustaceans(db: DbConn, _user: EditorUser) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::find_multiple(c, 100)
            .map(|rustacean| json!(rustacean))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}

#[get("/rustaceans/<id>")]
pub async fn view_rustacean(
    db: DbConn,
    _user: EditorUser,
    id: i32,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::find(&c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| match e {
                NotFound => Custom(Status::NotFound, json!("Rustacean not found")),
                _ => server_error(&e.into()),
            })
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    db: DbConn,
    _user: EditorUser,
    new_rustacean: Json<NewRustacean>,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| Custom(Status::Created, json!(rustacean)))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    db: DbConn,
    _user: EditorUser,
    id: i32,
    rustacean: Json<Rustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::save(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}

#[delete("/rustaceans/<id>")]
pub async fn delete_rustacean(
    db: DbConn,
    _user: EditorUser,
    id: i32,
) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}
