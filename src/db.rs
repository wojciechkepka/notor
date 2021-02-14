use super::models::Db;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::boxed::Box;
use std::env;
use std::sync::{Arc, Mutex};

pub fn db_connection() -> Result<Db, Box<dyn std::error::Error + Sync + Send + 'static>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    Ok(Arc::new(Mutex::new(PgConnection::establish(
        &database_url,
    )?)))
}
