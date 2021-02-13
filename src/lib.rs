#[macro_use]
extern crate diesel;

pub mod db;
mod handlers;
pub(crate) mod html;
mod models;
mod rejections;
mod routes;
mod web;

pub mod schema;
pub use routes::routes;
