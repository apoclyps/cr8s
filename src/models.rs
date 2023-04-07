use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{pg::Pg, sql_types::Text};

use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize)]
pub struct Rustacean {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "rustaceans"]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(Queryable, Associations, Serialize, Deserialize)]
pub struct Crate {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "crates"]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Identifiable, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Debug)]
pub struct Role {
    pub id: i32,
    pub code: RoleCode,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "roles"]
pub struct NewRole {
    pub code: RoleCode,
    pub name: String,
}

#[derive(Identifiable, Associations, Queryable, Debug)]
#[belongs_to(User)]
#[belongs_to(Role)]
#[table_name = "users_roles"]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Insertable)]
#[table_name = "users_roles"]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Debug, FromSqlRow, AsExpression)]
#[sql_type = "diesel::sql_types::Text"]
pub enum RoleCode {
    Admin,
    Editor,
    Viewer,
}

impl RoleCode {
    pub fn from_string(string: String) -> Result<Self, Box<dyn std::error::Error>> {
        match string.as_str() {
            "admin" => Ok(RoleCode::Admin),
            "editor" => Ok(RoleCode::Editor),
            "viewer" => Ok(RoleCode::Viewer),
            _ => Err("Invalid value to transform to Role code".into()),
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            RoleCode::Admin => "admin",
            RoleCode::Editor => "editor",
            RoleCode::Viewer => "viewer",
        }
    }
}

impl diesel::deserialize::FromSql<Text, Pg> for RoleCode {
    fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        match bytes {
            Some(b) => match std::str::from_utf8(b) {
                Ok("admin") => Ok(RoleCode::Admin),
                Ok("editor") => Ok(RoleCode::Editor),
                Ok("viewer") => Ok(RoleCode::Viewer),
                _ => Ok(RoleCode::Viewer),
            },
            _ => Ok(RoleCode::Viewer),
        }
    }
}

impl diesel::serialize::ToSql<Text, Pg> for RoleCode {
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, Pg>,
    ) -> diesel::serialize::Result {
        match *self {
            RoleCode::Admin => out.write_all(b"admin")?,
            RoleCode::Editor => out.write_all(b"editor")?,
            RoleCode::Viewer => out.write_all(b"viewer")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}
