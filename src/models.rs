use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub type Db = Arc<Mutex<PgConnection>>;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Note {
    pub note_id: i32,
    pub title: String,
    pub content: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub tag_id: i32,
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
