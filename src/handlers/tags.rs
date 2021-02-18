use super::*;
use crate::models::{NewTag, Tag};

pub(crate) async fn get_tags(
    filter: QueryFilter,
    username: String,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    Tag::load_tags(filter, username, &conn)
        .await
        .map(|tags| reply::json(&tags))
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn get_tag(id: i32, _: String, conn: Db) -> Result<impl Reply, Rejection> {
    Tag::load(id, &conn)
        .await
        .map(|tag| reply::json(&tag))
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn put_tag(tag: NewTag, _: String, conn: Db) -> Result<impl Reply, Rejection> {
    Tag::save(&tag, &conn)
        .await
        .map(|tag| reply::json(&tag))
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn delete_tag(id: i32, _: String, conn: Db) -> Result<impl Reply, Rejection> {
    Tag::delete(id, &conn)
        .await
        .map(|_| reply::reply())
        .map_err(RejectError::from)
        .map_err(reject::custom)
}
