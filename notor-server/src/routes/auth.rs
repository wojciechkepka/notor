use warp::body;
use warp::{Filter, Rejection, Reply};

use super::with_db;
use crate::db::Db;
use crate::handlers::auth::handle_login;

pub(crate) fn ro_auth(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("auth")
        .and(warp::post())
        .and(body::json())
        .and(with_db(db))
        .and_then(handle_login)
}
