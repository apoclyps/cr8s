use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{serde_json::json, Json, Value};

use crate::diesel::result::Error::NotFound;
use crate::models::{Crate, NewCrate};
use crate::repositories::CrateRepository;
use crate::rocket_routes::DbConn;

use super::server_error;

#[get("/crates")]
pub async fn get_crates(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        CrateRepository::find_multiple(c, 100)
            .map(|crates| json!(crates))
            .map_err(|e| server_error(&e.into()))
    })
    .await
}

#[get("/crates/<id>")]
pub async fn view_crate(db: DbConn, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::find(c, id)
            .map(|view_crate: Crate| json!(view_crate))
            .map_err(|e| match e {
                NotFound => Custom(Status::NotFound, json!({"error": "Crate not found"})),
                _ => server_error(&e.into()),
            })
    })
    .await
}

#[post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    db: DbConn,
    new_crate: Json<NewCrate>,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::create(c, new_crate.into_inner())
            .map(|new_crate| Custom(Status::Created, json!(new_crate)))
            .map_err(|e| server_error(&e.into()))
    })
    .await
}

#[put("/crates/<id>", format = "json", data = "<update_crate>")]
pub async fn update_crate(
    db: DbConn,
    id: i32,
    update_crate: Json<Crate>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::save(c, id, update_crate.into_inner())
            .map(|updated_crate| json!(updated_crate))
            .map_err(|e| server_error(&e.into()))
    })
    .await
}

#[delete("/crates/<id>")]
pub async fn delete_crate(db: DbConn, id: i32) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|e| server_error(&e.into()))
    })
    .await
}
