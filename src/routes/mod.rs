mod auth;
mod notes;
mod tags;
mod web;

use std::convert::Infallible;
use warp::{
    filters::header::headers_cloned,
    http::{HeaderMap, HeaderValue},
    Filter, Rejection, Reply,
};

use crate::db::Db;
use crate::handlers::auth::authorize;
use crate::models::UserRole;
use crate::rejections::handle_rejection;

use auth::*;
use notes::*;
use tags::*;
use web::*;

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn with_auth(
    role: UserRole,
    db: Db,
) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (role.clone(), db.clone(), headers))
        .and_then(authorize)
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

    let web_routes = ro_get_web(db.clone())
        .or(ro_web_note(db.clone()))
        .or(ro_web_tagview(db.clone()));

    let auth_routes = ro_auth(db.clone());

    let routes = notes_routes
        .or(tags_routes)
        .or(web_routes)
        .or(auth_routes)
        .recover(handle_rejection)
        .with(warp::log::log("route::notes"));

    return routes;
}
