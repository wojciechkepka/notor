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
        .and(warp::filters::query::query::<NotesFilter>())
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
    warp::path!("notes" / i32)
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

pub fn routes(db: Db) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    let notes_routes = ro_get_notes(db.clone())
        .or(ro_get_note(db.clone()))
        .or(ro_put_note(db.clone()))
        .or(ro_delete_note(db.clone()))
        .or(ro_update_note(db.clone()))
        .recover(handle_rejection)
        .with(warp::log::log("route::notes"));

    let routes = notes_routes;

    return routes;
}
