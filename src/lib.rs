#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate diesel_migrations;
extern crate r2d2;
extern crate rocket;

pub mod models;
pub mod schema;
pub mod connection;
