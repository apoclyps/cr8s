use rocket::{
    response::status::Custom,
    serde::json::{json, Json, Value},
};
use rocket_db_pools::Connection;

use crate::auth;
use crate::repositories::UserRepository;

use super::{server_error, CacheConn, DbConn};

#[post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    db: DbConn,
    cache: Connection<CacheConn>,
    credentials: Json<auth::Credentials>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        UserRepository::find_by_username(&c, &credentials.username)
            .map(|user| {
                if let Ok(token) = auth::authorize_user(&user, &credentials) {
                    return json!(token);
                }

                json!("Unauthorized")
            })
            .map_err(|e| server_error(&e.into()))
    })
    .await
}
