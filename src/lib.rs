#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

pub mod commands;
mod models;
mod repositories;
pub mod rocket_routes;
mod schema;
