mod auth;
mod notes;
mod tags;

use std::convert::Infallible;
use warp::{
    filters::cookie::cookie,
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue},
    Filter, Rejection, Reply,
};

use crate::auth::BEARER_COOKIE;
use crate::db::Db;
use crate::handlers::auth::{authorize_headers, authorize_token};
use crate::rejections::handle_rejection;
use notor_core::models::UserRole;

use auth::*;
use notes::*;
use tags::*;

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

#[allow(dead_code)]
fn with_auth_cookie(
    role: UserRole,
    db: Db,
) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    cookie(BEARER_COOKIE)
        .map(move |token: String| (role.clone(), db.clone(), token))
        .and_then(authorize_token)
}

pub fn with_auth_header(
    role: UserRole,
    db: Db,
) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (role.clone(), db.clone(), headers))
        .and_then(authorize_headers)
}

pub fn routes(db: Db) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    let notes_routes = ro_get_notes(db.clone())
        .or(ro_get_note(db.clone()))
        .or(ro_put_note(db.clone()))
        .or(ro_delete_note(db.clone()))
        .or(ro_update_note(db.clone()))
        .or(ro_tag_note(db.clone()))
        .or(ro_untag_note(db.clone()))
        .or(ro_get_note_tags(db.clone()));

    let tags_routes = ro_get_tags(db.clone())
        .or(ro_get_tag(db.clone()))
        .or(ro_put_tag(db.clone()))
        .or(ro_delete_tag(db.clone()));

    let auth_routes = ro_auth(db.clone());

    notes_routes
        .or(tags_routes)
        .or(auth_routes)
        .recover(handle_rejection)
        .with(warp::log("notor::routes"))
}
