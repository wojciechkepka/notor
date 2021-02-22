use crate::NotorError as Error;
use chrono::{Datelike, NaiveDateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};

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
    pub username: String,
    pub email: String,
    pub pass: String,
    pub role: UserRole,
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
}
