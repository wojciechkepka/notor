pub mod auth;
pub mod notes;
pub mod tags;

use crate::db::Db;
use warp::Rejection;

fn lock_db<'d>(db: &'d Db) -> Result<&'d Db, Rejection> {
    Ok(db)
}
