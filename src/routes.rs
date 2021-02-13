use super::handlers::*;
use super::models::Db;
use super::rejections::handle_rejection;
use std::convert::Infallible;
use warp::body;
use warp::{Filter, Rejection, Reply};

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn ro_get_notes(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("notes")
        .and(warp::get())
        .and(warp::filters::query::query::<QueryFilter>())
        .and(with_db(db))
        .and_then(get_notes)
}
pub fn ro_get_note(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("notes" / i32)
        .and(warp::get())
        .and(with_db(db))
        .and_then(get_note)
}
pub fn ro_put_note(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("notes")
        .and(warp::put())
        .and(body::json())
        .and(with_db(db))
        .and_then(put_note)
}
pub fn ro_delete_note(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("notes" / i32)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(delete_note)
}
pub fn ro_update_note(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("notes" / i32)
        .and(warp::post())
        .and(body::json())
        .and(with_db(db))
        .and_then(update_note)
}

pub fn ro_tag_note(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("notes" / i32 / "tags" / i32)
        .and(warp::post())
        .and(with_db(db))
        .and_then(tag_note)
}
pub fn ro_untag_note(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("notes" / i32 / "tags" / i32)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(untag_note)
}

pub fn ro_get_note_tags(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("notes" / i32 / "tags")
        .and(warp::get())
        .and(with_db(db))
        .and_then(get_note_tags)
}

pub fn ro_get_tags(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tags")
        .and(warp::get())
        .and(warp::filters::query::query::<QueryFilter>())
        .and(with_db(db))
        .and_then(get_tags)
}
pub fn ro_get_tag(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tags" / i32)
        .and(warp::get())
        .and(with_db(db))
        .and_then(get_tag)
}
pub fn ro_put_tag(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tags")
        .and(warp::put())
        .and(body::json())
        .and(with_db(db))
        .and_then(put_tag)
}
pub fn ro_delete_tag(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tags" / i32)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(delete_tag)
}

pub fn ro_get_web(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("web")
        .and(warp::get())
        .and(with_db(db))
        .and_then(get_web)
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

    let web_routes = ro_get_web(db.clone());

    let routes = notes_routes
        .or(tags_routes)
        .or(web_routes)
        .recover(handle_rejection)
        .with(warp::log::log("route::notes"));

    return routes;
}
