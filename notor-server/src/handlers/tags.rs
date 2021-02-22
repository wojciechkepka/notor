use warp::{reject, reply, Rejection, Reply};

use crate::db::Db;
use crate::filters::QueryFilter;
use crate::models::{delete_tag as _delete_tag, load_tag, save_tag, user_tags};
use notor_core::models::NewTag;

pub(crate) async fn get_tags(
    filter: QueryFilter,
    username: String,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    user_tags(filter, username, &conn)
        .await
        .map(|tags| reply::json(&tags))
        .map_err(reject::custom)
}

pub(crate) async fn get_tag(id: i32, _: String, conn: Db) -> Result<impl Reply, Rejection> {
    load_tag(id, &conn)
        .await
        .map(|tag| reply::json(&tag))
        .map_err(reject::custom)
}

pub(crate) async fn put_tag(tag: NewTag, _: String, conn: Db) -> Result<impl Reply, Rejection> {
    save_tag(&tag, &conn)
        .await
        .map(|tag| reply::json(&tag))
        .map_err(reject::custom)
}

pub(crate) async fn delete_tag(id: i32, _: String, conn: Db) -> Result<impl Reply, Rejection> {
    _delete_tag(id, &conn)
        .await
        .map(|_| reply::reply())
        .map_err(reject::custom)
}
