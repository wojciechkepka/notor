pub mod auth;
pub mod notes;
pub mod tags;
pub mod web;

use warp::{reject, reply, Rejection, Reply};

use crate::db::Db;
use crate::filters::QueryFilter;
use crate::rejections::*;
use crate::Error;

fn lock_db<'d>(db: &'d Db) -> Result<&'d Db, Rejection> {
    Ok(db)
}
