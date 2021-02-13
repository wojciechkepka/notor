use super::*;
use crate::handlers::web::*;

pub(crate) fn ro_get_web(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("web")
        .and(warp::get())
        .and(with_db(db))
        .and_then(get_web)
}
