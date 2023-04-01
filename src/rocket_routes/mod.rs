pub mod crates;
pub mod rustaceans;

use diesel::PgConnection;
use rocket_sync_db_pools::database;

#[database("postgres")]
pub struct DbConn(PgConnection);
