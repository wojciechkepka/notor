use super::models::Db;
use super::models::Note;
use super::models::Tag;
use super::rejections::{DbError, InvalidPayload, NotFound};
use super::schema;
use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;
use schema::tags::dsl::*;
use serde::Deserialize;
use warp::{reply, Rejection, Reply};

#[derive(Deserialize)]
pub(crate) struct NotesFilter {
    limit: Option<i64>,
}

pub(crate) async fn get_notes(filter: NotesFilter, conn: Db) -> Result<impl Reply, Rejection> {
    use schema::notes::dsl::*;
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    let limit = if let Some(l) = filter.limit {
        l
    } else {
        i64::MAX
    };

    Ok(reply::json(
        &notes
            .limit(limit)
            .load::<Note>(&*conn)
            .map_err(|_| NotFound::reject())?,
    ))
}

pub(crate) async fn get_note(id: i32, conn: Db) -> Result<impl Reply, Rejection> {
    use schema::notes::dsl::*;
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    Ok(reply::json(
        &notes
            .filter(note_id.eq(id))
            .first::<Note>(&*conn)
            .map_err(|_| NotFound::reject())?,
    ))
}

pub(crate) async fn put_note(id: i32, note: Note, conn: Db) -> Result<impl Reply, Rejection> {
    use schema::notes::dsl::*;
    if id != note.note_id {
        return Err(InvalidPayload::reject(
            "note_id does not match id from url path",
        ));
    }

    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    insert_into(notes)
        .values((
            note_id.eq(note.note_id),
            title.eq(&note.title),
            content.eq(&note.content),
        ))
        .execute(&*conn)
        .map_err(|e| InvalidPayload::reject(e))?;

    Ok(reply::json(&note))
}

pub(crate) async fn delete_note(id: i32, conn: Db) -> Result<impl Reply, Rejection> {
    use schema::notes::dsl::*;
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    delete(notes.filter(note_id.eq(id)))
        .execute(&*conn)
        .map_err(|e| InvalidPayload::reject(e))?;

    Ok(reply::reply())
}

pub(crate) async fn update_note(id: i32, note: Note, conn: Db) -> Result<impl Reply, Rejection> {
    use schema::notes::dsl::*;
    if id != note.note_id {
        return Err(InvalidPayload::reject(
            "note_id does not match id from url path",
        ));
    }

    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    update(notes)
        .filter(note_id.eq(id))
        .set((title.eq(note.title), content.eq(note.content)))
        .execute(&*conn)
        .map_err(|_| NotFound::reject())?;

    Ok(reply::reply())
}

pub(crate) async fn get_tags(filter: NotesFilter, conn: Db) -> Result<impl Reply, Rejection> {
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

pub(crate) async fn put_tag(id: i32, tag: Tag, conn: Db) -> Result<impl Reply, Rejection> {
    use schema::tags::dsl::*;
    if id != tag.tag_id {
        return Err(InvalidPayload::reject(
            "tag_id does not match id from url path",
        ));
    }

    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    insert_into(tags)
        .values((tag_id.eq(tag.tag_id), name.eq(&tag.name)))
        .execute(&*conn)
        .map_err(|e| InvalidPayload::reject(e))?;

    Ok(reply::json(&tag))
}

pub(crate) async fn delete_tag(id: i32, conn: Db) -> Result<impl Reply, Rejection> {
    use schema::tags::dsl::*;
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    delete(tags.filter(tag_id.eq(id)))
        .execute(&*conn)
        .map_err(|e| InvalidPayload::reject(e))?;

    Ok(reply::reply())
}

pub(crate) async fn tag_note(
    note_id_: i32,
    tag_id_: i32,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    use schema::notes_tags::dsl::*;
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    insert_into(notes_tags)
        .values((note_id.eq(note_id_), tag_id.eq(tag_id_)))
        .execute(&*conn)
        .map_err(|e| InvalidPayload::reject(e))?;

    Ok(reply::reply())
}

pub(crate) async fn untag_note(
    note_id_: i32,
    tag_id_: i32,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    use schema::notes_tags::dsl::*;
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    delete(notes_tags.filter(tag_id.eq(tag_id_).and(note_id.eq(note_id_))))
        .execute(&*conn)
        .map_err(|e| InvalidPayload::reject(e))?;

    Ok(reply::reply())
}

pub(crate) async fn get_note_tags(note_id_: i32, conn: Db) -> Result<impl Reply, Rejection> {
    use schema::notes_tags::dsl::*;
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    let tag_ids = notes_tags
        .filter(note_id.eq(note_id_))
        .select(tag_id)
        .load::<i32>(&*conn)
        .map_err(|_| NotFound::reject())?;

    Ok(reply::json(
        &tags
            .filter(schema::tags::tag_id.eq_any(tag_ids))
            .load::<Tag>(&*conn)
            .map_err(|_| NotFound::reject())?,
    ))
}
