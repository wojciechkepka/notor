pub mod notes;
pub mod tags;
pub mod web;

use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;
use serde::Deserialize;
use warp::{reply, Rejection, Reply};

use crate::models::Db;
use crate::rejections::*;
use crate::schema;

pub(crate) use notes::*;
pub(crate) use tags::*;
pub(crate) use web::*;

#[derive(Deserialize)]
pub(crate) struct QueryFilter {
    limit: Option<i64>,
}
