use super::*;
use crate::models::{NewTag, Tag};

pub(crate) async fn get_tags(filter: QueryFilter, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = lock_db(&conn)?;

    Tag::load_tags(filter, &conn)
        .map(|tags| reply::json(&tags))
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn get_tag(id: i32, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = lock_db(&conn)?;

    Tag::load(id, &conn)
        .map(|tag| reply::json(&tag))
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn put_tag(tag: NewTag, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = lock_db(&conn)?;

    Tag::save(&tag, &conn)
        .map(|tag| reply::json(&tag))
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn delete_tag(id: i32, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = lock_db(&conn)?;

    Tag::delete(id, &conn)
        .map(|_| reply::reply())
        .map_err(RejectError::from)
        .map_err(reject::custom)
}
