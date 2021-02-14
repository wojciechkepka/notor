use super::*;
use crate::models::{NewNote, Note};
use warp::reject;

pub(crate) async fn get_notes(filter: QueryFilter, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = lock_db(&conn)?;

    Note::load_notes(filter, &conn)
        .map(|note| reply::json(&note))
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn get_note(id: i32, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = lock_db(&conn)?;

    Note::load(id, &conn)
        .map(|note| reply::json(&note))
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn put_note(note: NewNote, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = lock_db(&conn)?;

    Note::save(&note, &conn)
        .map(|note| reply::json(&note))
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn delete_note(id: i32, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = lock_db(&conn)?;

    Note::delete(id, &conn)
        .map(|_| reply::reply())
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn update_note(id: i32, note: NewNote, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = lock_db(&conn)?;

    Note::update(id, &note, &conn)
        .map(|_| reply::reply())
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn tag_note(
    note_id_: i32,
    tag_id_: i32,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    let conn = lock_db(&conn)?;

    Note::tag(note_id_, tag_id_, &conn)
        .map(|_| reply::reply())
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn untag_note(
    note_id_: i32,
    tag_id_: i32,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    let conn = lock_db(&conn)?;

    Note::untag(note_id_, tag_id_, &conn)
        .map(|_| reply::reply())
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn get_note_tags(note_id_: i32, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = lock_db(&conn)?;

    Note::tags(note_id_, &conn)
        .map(|tags| reply::json(&tags))
        .map_err(RejectError::from)
        .map_err(reject::custom)
}
