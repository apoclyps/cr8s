use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{serde_json::json, Json, Value};

use crate::models::{Crate, NewCrate};
use crate::repositories::CrateRepository;
use crate::rocket_routes::DbConn;

#[get("/crates")]
pub async fn get_crates(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        CrateRepository::find_multiple(c, 100)
            .map(|crates| json!(crates))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}

#[get("/crates/<id>")]
pub async fn view_crate(db: DbConn, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::find(c, id)
            .map(|view_crate: Crate| json!(view_crate))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}

#[post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(db: DbConn, new_crate: Json<NewCrate>) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::create(c, new_crate.into_inner())
            .map(|created_crate: Crate| json!(created_crate))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
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
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}

#[delete("/crates/<id>")]
pub async fn delete_crate(db: DbConn, id: i32) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|_e| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}
