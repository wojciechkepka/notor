use super::*;
use crate::handlers::web::*;

pub(crate) fn ro_get_web(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("web")
        .and(warp::get())
        .and(with_db(db))
        .and_then(get_web)
}

pub(crate) fn ro_web_note(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("web" / "notes" / i32)
        .and(warp::get())
        .and(with_db(db))
        .and_then(get_web_note)
}

pub(crate) fn ro_web_tagview(
    db: Db,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("web" / "tags" / i32)
        .and(warp::get())
        .and(with_db(db))
        .and_then(get_web_tagview)
}
