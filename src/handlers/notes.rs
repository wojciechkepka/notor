use super::*;
use crate::models::{NewNote, Note};

pub(crate) async fn get_notes(filter: QueryFilter, conn: Db) -> Result<impl Reply, Rejection> {
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

pub(crate) async fn put_note(note: NewNote, conn: Db) -> Result<impl Reply, Rejection> {
    use schema::notes::dsl::*;
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    insert_into(notes)
        .values((title.eq(&note.title), content.eq(&note.content)))
        .get_result::<Note>(&*conn)
        .map(|note| reply::json(&note))
        .map_err(|e| InvalidPayload::reject(e))
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
    use crate::models::Tag;
    use schema::notes_tags::dsl::*;
    use schema::tags::dsl::tags;

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
