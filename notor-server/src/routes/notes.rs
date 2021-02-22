use warp::body;
use warp::{Filter, Rejection, Reply};

use super::{with_auth_header, with_db};
use crate::db::Db;
use crate::filters::QueryFilter;
use crate::handlers::notes::*;
use notor_core::models::UserRole;

pub(crate) fn ro_get_notes(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("notes")
        .and(warp::get())
        .and(warp::filters::query::query::<QueryFilter>())
        .and(with_auth_header(UserRole::User, db.clone()))
        .and(with_db(db))
        .and_then(get_notes)
}
pub(crate) fn ro_get_note(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("notes" / i32)
        .and(warp::get())
        .and(with_auth_header(UserRole::User, db.clone()))
        .and(with_db(db))
        .and_then(get_note)
}
pub(crate) fn ro_put_note(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("notes")
        .and(warp::put())
        .and(body::json())
        .and(with_auth_header(UserRole::User, db.clone()))
        .and(with_db(db))
        .and_then(put_note)
}
pub(crate) fn ro_delete_note(
    db: Db,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("notes" / i32)
        .and(warp::delete())
        .and(with_auth_header(UserRole::User, db.clone()))
        .and(with_db(db))
        .and_then(delete_note)
}
pub(crate) fn ro_update_note(
    db: Db,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("notes" / i32)
        .and(warp::post())
        .and(body::json())
        .and(with_auth_header(UserRole::User, db.clone()))
        .and(with_db(db))
        .and_then(update_note)
}

pub(crate) fn ro_tag_note(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("notes" / i32 / "tags" / String)
        .and(warp::post())
        .and(with_auth_header(UserRole::User, db.clone()))
        .and(with_db(db))
        .and_then(tag_note)
}
pub(crate) fn ro_untag_note(
    db: Db,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("notes" / i32 / "tags" / i32)
        .and(warp::delete())
        .and(with_auth_header(UserRole::User, db.clone()))
        .and(with_db(db))
        .and_then(untag_note)
}

pub(crate) fn ro_get_note_tags(
    db: Db,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("notes" / i32 / "tags")
        .and(warp::get())
        .and(with_auth_header(UserRole::User, db.clone()))
        .and(with_db(db))
        .and_then(get_note_tags)
}
