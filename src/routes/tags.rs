use super::*;
use crate::handlers::tags::*;

pub(crate) fn ro_get_tags(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tags")
        .and(warp::get())
        .and(warp::filters::query::query::<QueryFilter>())
        .and(with_db(db))
        .and_then(get_tags)
}
pub(crate) fn ro_get_tag(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tags" / i32)
        .and(warp::get())
        .and(with_db(db))
        .and_then(get_tag)
}
pub(crate) fn ro_put_tag(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tags")
        .and(warp::put())
        .and(body::json())
        .and(with_db(db))
        .and_then(put_tag)
}
pub(crate) fn ro_delete_tag(
    db: Db,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tags" / i32)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(delete_tag)
}
