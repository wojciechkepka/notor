pub mod notes;
pub mod tags;
pub mod web;

use std::sync::MutexGuard;
use warp::{reject, reply, Rejection, Reply};

use crate::filters::QueryFilter;
use crate::models::{Db, DbConn};
use crate::rejections::*;

fn lock_db<'d>(db: &'d Db) -> Result<MutexGuard<'d, DbConn>, Rejection> {
    db.lock()
        .map_err(|e| RejectError::DbConnError(e.to_string()))
        .map_err(reject::custom)
}
