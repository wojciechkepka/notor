use warp::body;
use warp::{Filter, Rejection, Reply};

use super::{with_auth_header, with_db};
use crate::db::Db;
use crate::filters::QueryFilter;
use crate::handlers::tags::*;
use notor_core::models::UserRole;

pub(crate) fn ro_get_tags(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tags")
        .and(warp::get())
        .and(warp::filters::query::query::<QueryFilter>())
        .and(with_auth_header(UserRole::User, db.clone()))
        .and(with_db(db))
        .and_then(get_tags)
}
pub(crate) fn ro_get_tag(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tags" / i32)
        .and(warp::get())
        .and(with_auth_header(UserRole::User, db.clone()))
        .and(with_db(db))
        .and_then(get_tag)
}
pub(crate) fn ro_put_tag(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tags")
        .and(warp::put())
        .and(body::json())
        .and(with_auth_header(UserRole::User, db.clone()))
        .and(with_db(db))
        .and_then(put_tag)
}
pub(crate) fn ro_delete_tag(
    db: Db,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tags" / i32)
        .and(warp::delete())
        .and(with_auth_header(UserRole::User, db.clone()))
        .and(with_db(db))
        .and_then(delete_tag)
}
