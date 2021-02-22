use super::lock_db;
use crate::models::{
    delete_note as del_note, load_note, load_notes, load_user_from_id, note_tags, save_note,
    save_tag, search_tag, tag_note as _tag_note, untag_note as _untag_note,
    update_note as upd_note,
};
use notor_core::models::{NewNote, NewTag};
use warp::{reject, reply, Rejection, Reply};

use crate::db::Db;
use crate::filters::QueryFilter;
use crate::Error;

pub(crate) async fn get_notes(
    filter: QueryFilter,
    username: String,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    load_notes(filter, username, &conn)
        .await
        .map(|note| reply::json(&note))
        .map_err(reject::custom)
}

pub(crate) async fn get_note(id: i32, username: String, conn: Db) -> Result<impl Reply, Rejection> {
    let note = load_note(id, &conn).await.map_err(reject::custom)?;

    let user = load_user_from_id(note.user_id, &conn)
        .await
        .map_err(reject::custom)?;

    if user.username != username {
        Err(reject::custom(Error::UnauthorizedAccess))
    } else {
        Ok(reply::json(&note))
    }
}

pub(crate) async fn put_note(note: NewNote, _: String, conn: Db) -> Result<impl Reply, Rejection> {
    save_note(&note, &conn)
        .await
        .map(|note| reply::json(&note))
        .map_err(reject::custom)
}

pub(crate) async fn delete_note(id: i32, _: String, conn: Db) -> Result<impl Reply, Rejection> {
    del_note(id, &conn)
        .await
        .map(|_| reply::reply())
        .map_err(reject::custom)
}

pub(crate) async fn update_note(
    id: i32,
    note: NewNote,
    _: String,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    upd_note(id, &note, &conn)
        .await
        .map(|_| reply::reply())
        .map_err(reject::custom)
}

pub(crate) async fn tag_note(
    note_id_: i32,
    tag: String,
    username: String,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    let tag_id_ = match search_tag(&tag, &username, &conn)
        .await
        .map_err(reject::custom)?
    {
        Some(id) => Ok(id),
        None => save_tag(
            &NewTag {
                name: tag,
                username,
            },
            &conn,
        )
        .await
        .map(|tag| tag.id),
    }
    .map_err(reject::custom)?;

    _tag_note(note_id_, tag_id_, &conn)
        .await
        .map(|_| reply::reply())
        .map_err(reject::custom)
}

pub(crate) async fn untag_note(
    note_id_: i32,
    tag_id_: i32,
    _: String,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    _untag_note(note_id_, tag_id_, &conn)
        .await
        .map(|_| reply::reply())
        .map_err(reject::custom)
}

pub(crate) async fn get_note_tags(
    note_id_: i32,
    _: String,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    let conn = lock_db(&conn)?;

    note_tags(note_id_, &conn)
        .await
        .map(|tags| reply::json(&tags))
        .map_err(reject::custom)
}
