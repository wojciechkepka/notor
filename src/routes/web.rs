use warp::{Filter, Rejection, Reply};

use super::{with_auth_cookie, with_db};
use crate::db::Db;
use crate::handlers::web::*;
use crate::models::UserRole;

pub(crate) fn ro_get_web(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("web")
        .and(warp::get())
        .and(with_auth_cookie(UserRole::User, db.clone()))
        .and(with_db(db))
        .and_then(get_web)
}

pub(crate) fn ro_web_note(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("web" / "notes" / i32)
        .and(warp::get())
        .and(with_auth_cookie(UserRole::User, db.clone()))
        .and(with_db(db))
        .and_then(get_web_note)
}

pub(crate) fn ro_web_tagview(
    db: Db,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("web" / "tags" / i32)
        .and(warp::get())
        .and(with_auth_cookie(UserRole::User, db.clone()))
        .and(with_db(db))
        .and_then(get_web_tagview)
}

pub(crate) fn ro_web_login() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("web" / "login")
        .and(warp::get())
        .and_then(get_web_login)
}
