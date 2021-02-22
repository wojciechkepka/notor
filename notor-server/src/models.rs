use crate::db::DbConn;
use crate::filters::QueryFilter;
use crate::Error;
use notor_core::models::*;

pub async fn load_notes<S: AsRef<str>>(
    filter: QueryFilter,
    username: S,
    conn: &DbConn,
) -> Result<Vec<Note>, Error> {
    let limit = if let Some(l) = filter.limit {
        l
    } else {
        i64::MAX
    };

    if let Some(tag) = filter.tag_id {
        sqlx::query_as!(
            Note,
            r#"
    SELECT notes.id, users.id as "user_id: _", notes.created, title, content
    FROM notes
    INNER JOIN notes_tags on notes_tags.note_id = notes.id
    INNER JOIN users on users.id = notes.user_id
    WHERE notes_tags.tag_id = $1 AND username = $2
    ORDER BY notes.id
    LIMIT $3
                "#,
            tag,
            username.as_ref(),
            limit
        )
        .fetch_all(conn)
        .await
        .map_err(Error::from)
    } else {
        sqlx::query_as!(
            Note,
            r#"
    SELECT notes.id, users.id as "user_id: _", notes.created, title, content
    FROM notes
    INNER JOIN users on users.id = notes.user_id
    WHERE username = $1
    ORDER BY notes.id
    LIMIT $2
                "#,
            username.as_ref(),
            limit
        )
        .fetch_all(conn)
        .await
        .map_err(Error::from)
    }
}

#[allow(dead_code)]
pub async fn load_notes_with_tags<S: AsRef<str>>(
    filter: QueryFilter,
    username: S,
    conn: &DbConn,
) -> Result<Vec<NoteWithTags>, Error> {
    let mut notes_with_tags = Vec::new();
    for note in load_notes(filter, username, &conn).await? {
        let tags = note_tags(note.id, &conn).await?;

        notes_with_tags.push((note, tags));
    }

    Ok(notes_with_tags)
}

pub async fn load_note(id: i32, conn: &DbConn) -> Result<Note, Error> {
    sqlx::query_as!(
        Note,
        "
SELECT *
FROM notes
WHERE id = $1
            ",
        id
    )
    .fetch_one(conn)
    .await
    .map_err(Error::from)
}

pub async fn save_note(note: &NewNote, conn: &DbConn) -> Result<Note, Error> {
    sqlx::query_as!(
        Note,
        "
INSERT INTO notes ( created, title, content, user_id )
VALUES ( $1, $2, $3, ( SELECT id FROM users WHERE users.username = $4 ) )
RETURNING *
            ",
        chrono::offset::Utc::now().naive_utc(),
        note.title,
        note.content,
        note.username,
    )
    .fetch_one(conn)
    .await
    .map_err(Error::from)
}

pub async fn delete_note(id: i32, conn: &DbConn) -> Result<(), Error> {
    clear_note_tags(id, &conn).await?;
    sqlx::query!(
        "
DELETE FROM notes
WHERE id = $1
            ",
        id
    )
    .execute(conn)
    .await
    .map(|_| ())
    .map_err(Error::from)
}

pub async fn clear_note_tags(id: i32, conn: &DbConn) -> Result<(), Error> {
    sqlx::query!(
        "
DELETE FROM notes_tags
WHERE note_id = $1
            ",
        id
    )
    .execute(conn)
    .await
    .map(|_| ())
    .map_err(Error::from)
}

pub async fn update_note(id: i32, new_note: &NewNote, conn: &DbConn) -> Result<(), Error> {
    sqlx::query!(
        "
UPDATE notes
SET ( title, content ) = ( $1, $2 )
WHERE id = $3
            ",
        new_note.title,
        new_note.content,
        id
    )
    .execute(conn)
    .await
    .map(|_| ())
    .map_err(Error::from)
}

pub async fn tag_note(note_id: i32, tag_id: i32, conn: &DbConn) -> Result<(), Error> {
    sqlx::query!(
        "
INSERT INTO notes_tags ( note_id, tag_id )
VALUES ( $1, $2 )
            ",
        note_id,
        tag_id
    )
    .execute(conn)
    .await
    .map(|_| ())
    .map_err(Error::from)
}

pub async fn untag_note(note_id: i32, tag_id: i32, conn: &DbConn) -> Result<(), Error> {
    sqlx::query!(
        "
DELETE FROM notes_tags
WHERE note_id = $1 AND tag_id = $2
            ",
        note_id,
        tag_id
    )
    .execute(conn)
    .await
    .map(|_| ())
    .map_err(Error::from)
}

pub async fn note_tags(note_id: i32, conn: &DbConn) -> Result<Vec<Tag>, Error> {
    sqlx::query_as!(
        Tag,
        r#"
SELECT tags.id, u.id as "user_id: _", name
FROM tags
INNER JOIN notes_tags AS nt ON nt.tag_id = tags.id
INNER JOIN users AS u ON u.id = tags.user_id
WHERE nt.note_id = $1
            "#,
        note_id
    )
    .fetch_all(conn)
    .await
    .map_err(Error::from)
}

pub async fn user_tags<S: AsRef<str>>(
    filter: QueryFilter,
    username: S,
    conn: &DbConn,
) -> Result<Vec<Tag>, Error> {
    let limit = if let Some(l) = filter.limit {
        l
    } else {
        i64::MAX
    };

    sqlx::query_as!(
        Tag,
        r#"
SELECT tags.id, users.id as "user_id: _", name
FROM tags
INNER JOIN users on users.id = tags.user_id
WHERE users.username = $1
LIMIT $2
            "#,
        username.as_ref(),
        limit
    )
    .fetch_all(conn)
    .await
    .map_err(Error::from)
}

pub async fn load_tag(id: i32, conn: &DbConn) -> Result<Tag, Error> {
    sqlx::query_as!(
        Tag,
        r#"
SELECT id, name, user_id as "user_id: _"
FROM tags
WHERE id = $1
            "#,
        id
    )
    .fetch_one(conn)
    .await
    .map_err(Error::from)
}

pub async fn save_tag(tag: &NewTag, conn: &DbConn) -> Result<Tag, Error> {
    sqlx::query_as!(
        Tag,
        r#"
INSERT INTO tags ( name, user_id )
VALUES ( $1, (SELECT id FROM users WHERE username = $2) )
RETURNING id, name, user_id as "user_id: _" 
            "#,
        tag.name,
        tag.username,
    )
    .fetch_one(conn)
    .await
    .map_err(Error::from)
}

pub async fn delete_tag(id: i32, conn: &DbConn) -> Result<(), Error> {
    sqlx::query!(
        "
DELETE FROM tags
WHERE id = $1
            ",
        id
    )
    .execute(conn)
    .await
    .map_err(Error::from)
    .map(|_| ())
}

pub async fn search_tag<S: AsRef<str>>(
    tag: S,
    username: S,
    conn: &DbConn,
) -> Result<Option<i32>, Error> {
    sqlx::query!(
        "
SELECT tags.id
FROM tags
INNER JOIN users ON users.id = tags.user_id
WHERE name = $1 AND username = $2
            ",
        tag.as_ref(),
        username.as_ref()
    )
    .fetch_optional(conn)
    .await
    .map_err(Error::from)
    .map(|maybe| maybe.map(|record| record.id))
}

pub async fn load_user<S: AsRef<str>>(username: S, conn: &DbConn) -> Result<User, Error> {
    sqlx::query_as!(
        User,
        r#"
SELECT id, created, username, email, pass, role as "role: _"
FROM users
WHERE username = $1
            "#,
        username.as_ref()
    )
    .fetch_one(conn)
    .await
    .map_err(Error::from)
}

pub async fn load_user_from_id(id: i32, conn: &DbConn) -> Result<User, Error> {
    sqlx::query_as!(
        User,
        r#"
SELECT id, created, username, email, pass, role as "role: _"
FROM users
WHERE id = $1
            "#,
        id
    )
    .fetch_one(conn)
    .await
    .map_err(Error::from)
}

#[allow(dead_code)]
pub async fn save_user(user: &NewUser, conn: &DbConn) -> Result<User, Error> {
    sqlx::query_as!(
        User,
        r#"
INSERT INTO users ( username, email, pass, role )
VALUES ( $1, $2, $3, $4 )
RETURNING id, created, username, email, pass, role as "role: _"
            "#,
        &user.username,
        &user.email,
        &user.pass,
        user.role as _,
    )
    .fetch_one(conn)
    .await
    .map_err(Error::from)
}

#[allow(dead_code)]
pub async fn delete_user<S: AsRef<str>>(username: S, conn: &DbConn) -> Result<(), Error> {
    sqlx::query!(
        "
DELETE FROM users
WHERE username = $1
            ",
        username.as_ref()
    )
    .execute(conn)
    .await
    .map_err(Error::from)
    .map(|_| ())
}

pub async fn load_claims<S: AsRef<str>>(sub: S, conn: &DbConn) -> Result<Claims, Error> {
    sqlx::query_as!(
        Claims,
        "
SELECT sub, role, exp
FROM claims
WHERE sub = $1
            ",
        sub.as_ref()
    )
    .fetch_one(conn)
    .await
    .map_err(Error::from)
}

pub async fn load_claims_if_exists<S: AsRef<str>>(sub: S, conn: &DbConn) -> Option<Claims> {
    load_claims(sub, conn).await.ok()
}

pub async fn save_claims(claim: &Claims, conn: &DbConn) -> Result<Claims, Error> {
    sqlx::query_as!(
        Claims,
        "
INSERT INTO claims ( sub, role, exp )
VALUES ( $1, $2, $3 )
RETURNING *
            ",
        &claim.sub,
        &claim.role,
        &claim.exp,
    )
    .fetch_one(conn)
    .await
    .map_err(Error::from)
}

pub async fn delete_claims<S: AsRef<str>>(sub: S, conn: &DbConn) -> Result<(), Error> {
    sqlx::query!(
        "
DELETE FROM claims
WHERE sub = $1
            ",
        sub.as_ref()
    )
    .execute(conn)
    .await
    .map_err(Error::from)
    .map(|_| ())
}
