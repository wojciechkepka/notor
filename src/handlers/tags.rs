use super::*;
use crate::models::{NewTag, Tag};

pub(crate) async fn get_tags(filter: QueryFilter, conn: Db) -> Result<impl Reply, Rejection> {
    use schema::tags::dsl::*;
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    let limit = if let Some(l) = filter.limit {
        l
    } else {
        i64::MAX
    };

    Ok(reply::json(
        &tags
            .limit(limit)
            .load::<Tag>(&*conn)
            .map_err(|_| NotFound::reject())?,
    ))
}

pub(crate) async fn get_tag(id: i32, conn: Db) -> Result<impl Reply, Rejection> {
    use schema::tags::dsl::*;
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    Ok(reply::json(
        &tags
            .filter(tag_id.eq(id))
            .first::<Tag>(&*conn)
            .map_err(|_| NotFound::reject())?,
    ))
}

pub(crate) async fn put_tag(tag: NewTag, conn: Db) -> Result<impl Reply, Rejection> {
    use schema::tags::dsl::*;
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    insert_into(tags)
        .values(name.eq(&tag.name))
        .get_result::<Tag>(&*conn)
        .map(|tag| reply::json(&tag))
        .map_err(|e| InvalidPayload::reject(e))
}

pub(crate) async fn delete_tag(id: i32, conn: Db) -> Result<impl Reply, Rejection> {
    use schema::tags::dsl::*;
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    delete(tags.filter(tag_id.eq(id)))
        .execute(&*conn)
        .map_err(|e| InvalidPayload::reject(e))?;

    Ok(reply::reply())
}
