use crate::db::DbConn;
use crate::filters::QueryFilter;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Error as DbErr;

pub type NoteWithTags = (Note, Vec<Tag>);

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: i32,
    pub created: NaiveDateTime,
    pub title: String,
    pub content: Option<String>,
}

impl Note {
    pub async fn load_notes(filter: QueryFilter, conn: &DbConn) -> Result<Vec<Note>, DbErr> {
        let limit = if let Some(l) = filter.limit {
            l
        } else {
            i64::MAX
        };

        sqlx::query_as!(
            Note,
            "
SELECT *
FROM notes
ORDER BY id
LIMIT $1
            ",
            limit
        )
        .fetch_all(conn)
        .await
    }

    pub async fn load_notes_with_tags(
        filter: QueryFilter,
        conn: &DbConn,
    ) -> Result<Vec<NoteWithTags>, DbErr> {
        let mut notes_with_tags = Vec::new();
        for note in Note::load_notes(filter, &conn).await? {
            let tags = Note::tags(note.id, &conn).await?;

            notes_with_tags.push((note, tags));
        }

        Ok(notes_with_tags)
    }

    pub async fn load(id: i32, conn: &DbConn) -> Result<Note, DbErr> {
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
    }

    pub async fn save(note: &NewNote, conn: &DbConn) -> Result<Note, DbErr> {
        sqlx::query_as!(
            Note,
            "
INSERT INTO notes ( created, title, content )
VALUES ( $1, $2, $3 )
RETURNING *
            ",
            chrono::offset::Utc::now().naive_utc(),
            note.title,
            note.content
        )
        .fetch_one(conn)
        .await
    }

    pub async fn delete(id: i32, conn: &DbConn) -> Result<(), DbErr> {
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
    }

    pub async fn update(id: i32, new_note: &NewNote, conn: &DbConn) -> Result<(), DbErr> {
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
    }

    pub async fn tag(note_id: i32, tag_id: i32, conn: &DbConn) -> Result<(), DbErr> {
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
    }

    pub async fn untag(note_id: i32, tag_id: i32, conn: &DbConn) -> Result<(), DbErr> {
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
    }

    pub async fn tags(note_id: i32, conn: &DbConn) -> Result<Vec<Tag>, DbErr> {
        sqlx::query_as!(
            Tag,
            "
SELECT id, name
FROM tags
INNER JOIN notes_tags AS nt ON nt.tag_id = tags.id
WHERE nt.note_id = $1
            ",
            note_id
        )
        .fetch_all(conn)
        .await
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewNote {
    pub title: String,
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

impl Tag {
    pub async fn load_tags(filter: QueryFilter, conn: &DbConn) -> Result<Vec<Tag>, DbErr> {
        let limit = if let Some(l) = filter.limit {
            l
        } else {
            i64::MAX
        };

        sqlx::query_as!(
            Tag,
            "
SELECT *
FROM tags
LIMIT $1
            ",
            limit
        )
        .fetch_all(conn)
        .await
    }

    pub async fn load(id: i32, conn: &DbConn) -> Result<Tag, DbErr> {
        sqlx::query_as!(
            Tag,
            "
SELECT *
FROM tags
WHERE id = $1
            ",
            id
        )
        .fetch_one(conn)
        .await
    }

    pub async fn save(tag: &NewTag, conn: &DbConn) -> Result<Tag, DbErr> {
        sqlx::query_as!(
            Tag,
            "
INSERT INTO tags ( name )
VALUES ( $1 )
RETURNING *
            ",
            tag.name,
        )
        .fetch_one(conn)
        .await
    }

    pub async fn delete(id: i32, conn: &DbConn) -> Result<(), DbErr> {
        sqlx::query!(
            "
DELETE FROM tags
WHERE id = $1
            ",
            id
        )
        .execute(conn)
        .await
        .map(|_| ())
    }

    pub async fn search<S: AsRef<str>>(tag: S, conn: &DbConn) -> Result<Option<i32>, DbErr> {
        sqlx::query!(
            "
SELECT id
FROM tags
WHERE name = $1
            ",
            tag.as_ref()
        )
        .fetch_optional(conn)
        .await
        .map(|maybe| maybe.map(|record| record.id))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTag {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct ErrReply {
    pub message: String,
}
impl ErrReply {
    pub fn new<S: AsRef<str>>(message: S) -> ErrReply {
        ErrReply {
            message: message.as_ref().to_string(),
        }
    }
}
