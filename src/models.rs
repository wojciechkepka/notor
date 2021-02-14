use crate::filters::QueryFilter;
use crate::schema::{self, notes, tags};
use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;
use diesel::{pg::PgConnection, result::Error as DbErr};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub type DbConn = PgConnection;
pub type Db = Arc<Mutex<DbConn>>;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Note {
    pub note_id: i32,
    pub title: String,
    pub content: Option<String>,
}

impl Note {
    pub fn load_notes(filter: QueryFilter, conn: &DbConn) -> Result<Vec<Note>, DbErr> {
        use schema::notes::dsl::*;

        let limit = if let Some(l) = filter.limit {
            l
        } else {
            i64::MAX
        };

        notes.limit(limit).load::<Note>(&*conn)
    }

    pub fn load(id: i32, conn: &DbConn) -> Result<Note, DbErr> {
        use schema::notes::dsl::*;
        notes.filter(note_id.eq(id)).first::<Note>(&*conn)
    }

    pub fn save(note: &NewNote, conn: &DbConn) -> Result<Note, DbErr> {
        use schema::notes::dsl::*;
        insert_into(notes)
            .values((title.eq(&note.title), content.eq(&note.content)))
            .get_result::<Note>(&*conn)
    }

    pub fn delete(id: i32, conn: &DbConn) -> Result<usize, DbErr> {
        use schema::notes::dsl::*;
        delete(notes.filter(note_id.eq(id))).execute(&*conn)
    }

    pub fn update(id: i32, new_note: &NewNote, conn: &DbConn) -> Result<usize, DbErr> {
        use schema::notes::dsl::*;

        update(notes)
            .filter(note_id.eq(id))
            .set((title.eq(&new_note.title), content.eq(&new_note.content)))
            .execute(&*conn)
    }

    pub fn tag(note_id_: i32, tag_id_: i32, conn: &DbConn) -> Result<usize, DbErr> {
        use schema::notes_tags::dsl::*;

        insert_into(notes_tags)
            .values((note_id.eq(note_id_), tag_id.eq(tag_id_)))
            .execute(&*conn)
    }

    pub fn untag(note_id_: i32, tag_id_: i32, conn: &DbConn) -> Result<usize, DbErr> {
        use schema::notes_tags::dsl::*;

        delete(notes_tags.filter(tag_id.eq(tag_id_).and(note_id.eq(note_id_)))).execute(&*conn)
    }

    pub fn tags(note_id_: i32, conn: &DbConn) -> Result<Vec<Tag>, DbErr> {
        use schema::notes_tags::dsl::*;
        use schema::tags::dsl::tags;

        let tag_ids = notes_tags
            .filter(note_id.eq(note_id_))
            .select(tag_id)
            .load::<i32>(&*conn)?;

        tags.filter(schema::tags::tag_id.eq_any(tag_ids))
            .load::<Tag>(&*conn)
    }
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "notes"]
pub struct NewNote {
    pub title: String,
    pub content: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub tag_id: i32,
    pub name: String,
}

impl Tag {
    pub fn load_tags(filter: QueryFilter, conn: &DbConn) -> Result<Vec<Tag>, DbErr> {
        use schema::tags::dsl::*;
        let limit = if let Some(l) = filter.limit {
            l
        } else {
            i64::MAX
        };

        tags.limit(limit).load::<Tag>(&*conn)
    }

    pub fn load(id: i32, conn: &DbConn) -> Result<Tag, DbErr> {
        use schema::tags::dsl::*;

        tags.filter(tag_id.eq(id)).first::<Tag>(&*conn)
    }

    pub fn save(tag: &NewTag, conn: &DbConn) -> Result<Tag, DbErr> {
        use schema::tags::dsl::*;

        insert_into(tags)
            .values(name.eq(&tag.name))
            .get_result::<Tag>(&*conn)
    }

    pub fn delete(id: i32, conn: &DbConn) -> Result<usize, DbErr> {
        use schema::tags::dsl::*;

        delete(tags.filter(tag_id.eq(id))).execute(&*conn)
    }
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "tags"]
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
