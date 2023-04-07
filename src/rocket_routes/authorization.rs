use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{json, Json, Value},
};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};

use crate::repositories::UserRepository;
use crate::{auth, models::User};

use super::{server_error, CacheConn, DbConn};
use diesel::result::Error::NotFound;

#[post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    db: DbConn,
    mut cache: Connection<CacheConn>,
    credentials: Json<auth::Credentials>,
) -> Result<Value, Custom<Value>> {
    let username: String = credentials.username.clone();
    let user: User = db
        .run(move |c| {
            UserRepository::find_by_username(&c, &username).map_err(|e| match e {
                NotFound => Custom(Status::Unauthorized, json!("Wrong credentials")),
                _ => server_error(&e.into()),
            })
        })
        .await?;

    let session_id: String = auth::authorize_user(&user, &credentials)
        .map_err(|_| Custom(Status::Unauthorized, json!("Wrong credentials")))?;

    cache
        .set_ex::<_, _, ()>(format!("sessions/{}", session_id), user.id, 3 * 60 * 60)
        .await
        .map(|_| json!({ "token": session_id }))
        .map_err(|e| server_error(&e.into()))
}
