use super::*;
use crate::models::{NewTag, Tag};

pub(crate) async fn get_tags(filter: QueryFilter, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    Tag::load_tags(filter, &conn)
        .map(|tags| reply::json(&tags))
        .map_err(NotFound::reject)
}

pub(crate) async fn get_tag(id: i32, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    Tag::load(id, &conn)
        .map(|tag| reply::json(&tag))
        .map_err(NotFound::reject)
}

pub(crate) async fn put_tag(tag: NewTag, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    Tag::save(&tag, &conn)
        .map(|tag| reply::json(&tag))
        .map_err(|e| InvalidPayload::reject(e))
}

pub(crate) async fn delete_tag(id: i32, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    Tag::delete(id, &conn)
        .map(|_| reply::reply())
        .map_err(|e| InvalidPayload::reject(e))
}
