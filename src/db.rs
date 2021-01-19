use super::models::Db;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::sync::{Arc, Mutex};

pub fn db_connection() -> Db {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Arc::new(Mutex::new(
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url)),
    ))
}
