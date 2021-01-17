use super::db::db_connection;
use super::models::Note;
use super::rejections::{InvalidNote, NotFound};
use super::schema;
use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;
use schema::notes::dsl::*;
use serde::Deserialize;
use warp::{reply, Rejection, Reply};

#[derive(Deserialize)]
pub(crate) struct NotesFilter {
    limit: Option<i64>,
}

pub(crate) async fn get_notes(filter: NotesFilter) -> Result<impl Reply, Rejection> {
    let conn = db_connection();

    let limit = if let Some(l) = filter.limit {
        l
    } else {
        i64::MAX
    };

    Ok(reply::json(
        &notes
            .limit(limit)
            .load::<Note>(&conn)
            .map_err(|_| NotFound::reject())?,
    ))
}

pub(crate) async fn get_note(id: i32) -> Result<impl Reply, Rejection> {
    let conn = db_connection();

    Ok(reply::json(
        &notes
            .filter(note_id.eq(id))
            .first::<Note>(&conn)
            .map_err(|_| NotFound::reject())?,
    ))
}

pub(crate) async fn put_note(id: i32, note: Note) -> Result<impl Reply, Rejection> {
    let conn = db_connection();

    if id != note.note_id {
        return Err(InvalidNote::reject(
            "note_id does not match id from url path",
        ));
    }

    insert_into(notes)
        .values((
            note_id.eq(note.note_id),
            title.eq(&note.title),
            content.eq(&note.content),
        ))
        .execute(&conn)
        .map_err(|e| InvalidNote::reject(e))?;

    Ok(reply::json(&note))
}

pub(crate) async fn delete_note(id: i32) -> Result<impl Reply, Rejection> {
    let conn = db_connection();

    delete(notes.filter(note_id.eq(id)))
        .execute(&conn)
        .map_err(|e| InvalidNote::reject(e))?;

    Ok(reply::reply())
}

pub(crate) async fn update_note(id: i32, note: Note) -> Result<impl Reply, Rejection> {
    let conn = db_connection();

    if id != note.note_id {
        return Err(InvalidNote::reject(
            "note_id does not match id from url path",
        ));
    }

    update(notes)
        .filter(note_id.eq(id))
        .set((title.eq(note.title), content.eq(note.content)))
        .execute(&conn)
        .map_err(|_| NotFound::reject())?;

    Ok(reply::reply())
}
