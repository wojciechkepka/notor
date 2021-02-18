use crate::db::DbConn;
use crate::filters::QueryFilter;
use crate::Error;

use chrono::{Datelike, NaiveDateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Error as DbErr;

pub type NoteWithTags = (Note, Vec<Tag>);

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: i32,
    pub user_id: i32,
    pub created: NaiveDateTime,
    pub title: String,
    pub content: Option<String>,
}

impl Note {
    pub fn created_datetime(&self) -> String {
        let created = &self.created;
        format!(
            "{}-{:02}-{:02} {:02}:{:02}",
            created.year(),
            created.month(),
            created.day(),
            created.hour(),
            created.minute()
        )
    }
    pub async fn load_notes<S: AsRef<str>>(
        filter: QueryFilter,
        username: S,
        conn: &DbConn,
    ) -> Result<Vec<Note>, DbErr> {
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
        }
    }

    pub async fn load_notes_with_tags<S: AsRef<str>>(
        filter: QueryFilter,
        username: S,
        conn: &DbConn,
    ) -> Result<Vec<NoteWithTags>, DbErr> {
        let mut notes_with_tags = Vec::new();
        for note in Note::load_notes(filter, username, &conn).await? {
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
    }

    pub async fn delete(id: i32, conn: &DbConn) -> Result<(), DbErr> {
        Self::clear_tags(id, &conn).await?;
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

    pub async fn clear_tags(id: i32, conn: &DbConn) -> Result<(), DbErr> {
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
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewNote {
    pub username: String,
    pub title: String,
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Tag {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
}

impl Tag {
    pub async fn load_tags<S: AsRef<str>>(
        filter: QueryFilter,
        username: S,
        conn: &DbConn,
    ) -> Result<Vec<Tag>, DbErr> {
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
    }

    pub async fn load(id: i32, conn: &DbConn) -> Result<Tag, DbErr> {
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
    }

    pub async fn save(tag: &NewTag, conn: &DbConn) -> Result<Tag, DbErr> {
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

    pub async fn search<S: AsRef<str>>(
        tag: S,
        username: S,
        conn: &DbConn,
    ) -> Result<Option<i32>, DbErr> {
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
        .map(|maybe| maybe.map(|record| record.id))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTag {
    pub name: String,
    pub username: String,
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
}

impl std::str::FromStr for UserRole {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(UserRole::User),
            "admin" => Ok(UserRole::Admin),
            role => Err(Error::InvalidRole(role.to_string())),
        }
    }
}

impl AsRef<str> for UserRole {
    fn as_ref(&self) -> &str {
        match self {
            UserRole::User => "user",
            UserRole::Admin => "admin",
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub created: NaiveDateTime,
    pub username: String,
    pub email: String,
    pub pass: String,
    pub role: UserRole,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewUser {
    username: String,
    email: String,
    pass: String,
    role: UserRole,
}

impl User {
    pub async fn load<S: AsRef<str>>(username: S, conn: &DbConn) -> Result<Self, DbErr> {
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
    }

    pub async fn load_id(id: i32, conn: &DbConn) -> Result<Self, DbErr> {
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
    }

    #[allow(dead_code)]
    pub async fn save(user: &NewUser, conn: &DbConn) -> Result<Self, DbErr> {
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
    }

    #[allow(dead_code)]
    pub async fn delete<S: AsRef<str>>(username: S, conn: &DbConn) -> Result<(), DbErr> {
        sqlx::query!(
            "
DELETE FROM users
WHERE username = $1
            ",
            username.as_ref()
        )
        .execute(conn)
        .await
        .map(|_| ())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct JsonAuth {
    pub username: String,
    pub pass: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: i64,
}

impl Claims {
    pub fn is_expired(&self) -> bool {
        self.exp < Utc::now().timestamp()
    }
    pub async fn load<S: AsRef<str>>(sub: S, conn: &DbConn) -> Result<Self, DbErr> {
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
    }

    pub async fn load_if_exists<S: AsRef<str>>(sub: S, conn: &DbConn) -> Option<Self> {
        Self::load(sub, conn).await.ok()
    }

    pub async fn save(claim: &Claims, conn: &DbConn) -> Result<Self, DbErr> {
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
    }

    pub async fn delete<S: AsRef<str>>(sub: S, conn: &DbConn) -> Result<(), DbErr> {
        sqlx::query!(
            "
DELETE FROM claims
WHERE sub = $1
            ",
            sub.as_ref()
        )
        .execute(conn)
        .await
        .map(|_| ())
    }
}
