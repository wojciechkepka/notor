pub mod notes;
pub mod tags;
pub mod web;

use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;
use warp::{reply, Rejection, Reply};

use crate::filters::QueryFilter;
use crate::models::Db;
use crate::rejections::*;
use crate::schema;
