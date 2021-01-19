#[macro_use]
extern crate diesel;

pub mod db;
mod handlers;
mod models;
mod rejections;
mod routes;

pub mod schema;
pub use routes::routes;
