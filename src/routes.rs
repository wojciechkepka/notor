use super::handlers::*;
use super::rejections::handle_rejection;
use std::convert::Infallible;
use warp::body;
use warp::{Filter, Reply};

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    let get_notes = warp::path!("notes")
        .and(warp::get())
        .and(warp::filters::query::query::<NotesFilter>())
        .and_then(get_notes);

    let get_note = warp::path!("notes" / i32)
        .and(warp::get())
        .and_then(get_note);

    let put_note = warp::path!("notes" / i32)
        .and(warp::put())
        .and(body::json())
        .and_then(put_note);

    let delete_note = warp::path!("notes" / i32)
        .and(warp::delete())
        .and_then(delete_note);

    let update_note = warp::path!("notes" / i32)
        .and(warp::post())
        .and(body::json())
        .and_then(update_note);

    let notes_routes = get_note
        .or(get_notes)
        .or(put_note)
        .or(delete_note)
        .or(update_note)
        .recover(handle_rejection)
        .with(warp::log::log("route::notes"));

    let routes = notes_routes;

    return routes;
}
