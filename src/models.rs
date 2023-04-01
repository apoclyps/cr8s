use chrono::NaiveDateTime;
use crate::schema::*;

#[derive(Queryable)]
struct Rustacean {
    id: i32,
    name: String,
    email: String,
    created_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name="rustaceans"]
struct NewRustacean {
    name: String,
    email: String
}

#[derive(Queryable, Associations)]
struct Crate {
    id: i32,
    rustacean_id: i32,
    code: String,
    name: String,
    version: String,
    description: Option<String>,
    created_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name="crates"]
struct NewCrate {
    rustacean_id: i32,
    code: String,
    name: String,
    version: String,
    description: Option<String>,
}
