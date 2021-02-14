use super::*;
use crate::models::{NewNote, Note};

pub(crate) async fn get_notes(filter: QueryFilter, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    Note::load_notes(filter, &conn)
        .map(|note| reply::json(&note))
        .map_err(NotFound::reject)
}

pub(crate) async fn get_note(id: i32, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    Note::load(id, &conn)
        .map(|note| reply::json(&note))
        .map_err(NotFound::reject)
}

pub(crate) async fn put_note(note: NewNote, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    Note::save(&note, &conn)
        .map(|note| reply::json(&note))
        .map_err(InvalidPayload::reject)
}

pub(crate) async fn delete_note(id: i32, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    Note::delete(id, &conn)
        .map(|_| reply::reply())
        .map_err(InvalidPayload::reject)
}

pub(crate) async fn update_note(id: i32, note: NewNote, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    Note::update(id, &note, &conn)
        .map(|_| reply::reply())
        .map_err(NotFound::reject)
}

pub(crate) async fn tag_note(
    note_id_: i32,
    tag_id_: i32,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    Note::tag(note_id_, tag_id_, &conn)
        .map(|_| reply::reply())
        .map_err(InvalidPayload::reject)
}

pub(crate) async fn untag_note(
    note_id_: i32,
    tag_id_: i32,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    Note::untag(note_id_, tag_id_, &conn)
        .map(|_| reply::reply())
        .map_err(InvalidPayload::reject)
}

pub(crate) async fn get_note_tags(note_id_: i32, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    Note::tags(note_id_, &conn)
        .map(|tags| reply::json(&tags))
        .map_err(NotFound::reject)
}
