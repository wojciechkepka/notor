use super::*;
use crate::models::{NewNote, NewTag, Note, Tag, User};
use crate::Error;
use warp::reject;

pub(crate) async fn get_notes(
    filter: QueryFilter,
    username: String,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    Note::load_notes(filter, username, &conn)
        .await
        .map(|note| reply::json(&note))
        .map_err(Error::from)
        .map_err(reject::custom)
}

pub(crate) async fn get_note(id: i32, username: String, conn: Db) -> Result<impl Reply, Rejection> {
    let note = Note::load(id, &conn)
        .await
        .map_err(Error::from)
        .map_err(reject::custom)?;

    let user = User::load_id(note.user_id, &conn)
        .await
        .map_err(Error::from)
        .map_err(reject::custom)?;

    if user.username != username {
        Err(reject::custom(Error::UnauthorizedAccess))
    } else {
        Ok(reply::json(&note))
    }
}

pub(crate) async fn put_note(note: NewNote, _: String, conn: Db) -> Result<impl Reply, Rejection> {
    Note::save(&note, &conn)
        .await
        .map(|note| reply::json(&note))
        .map_err(Error::from)
        .map_err(reject::custom)
}

pub(crate) async fn delete_note(id: i32, _: String, conn: Db) -> Result<impl Reply, Rejection> {
    Note::delete(id, &conn)
        .await
        .map(|_| reply::reply())
        .map_err(Error::from)
        .map_err(reject::custom)
}

pub(crate) async fn update_note(
    id: i32,
    note: NewNote,
    _: String,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    Note::update(id, &note, &conn)
        .await
        .map(|_| reply::reply())
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn tag_note(
    note_id_: i32,
    tag: String,
    username: String,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    let tag_id_ = match Tag::search(&tag, &username, &conn)
        .await
        .map_err(RejectError::from)
        .map_err(reject::custom)?
    {
        Some(id) => Ok(id),
        None => Tag::save(
            &NewTag {
                name: tag,
                username,
            },
            &conn,
        )
        .await
        .map(|tag| tag.id),
    }
    .map_err(RejectError::from)
    .map_err(reject::custom)?;

    Note::tag(note_id_, tag_id_, &conn)
        .await
        .map(|_| reply::reply())
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn untag_note(
    note_id_: i32,
    tag_id_: i32,
    _: String,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    Note::untag(note_id_, tag_id_, &conn)
        .await
        .map(|_| reply::reply())
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn get_note_tags(
    note_id_: i32,
    _: String,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    let conn = lock_db(&conn)?;

    Note::tags(note_id_, &conn)
        .await
        .map(|tags| reply::json(&tags))
        .map_err(RejectError::from)
        .map_err(reject::custom)
}
