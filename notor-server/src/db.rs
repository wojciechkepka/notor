use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::boxed::Box;
use std::env;
use std::sync::Arc;

pub type DbConn = PgPool;
pub type Db = Arc<PgPool>;

const MAX_CONNECTIONS: u32 = 5;

pub async fn db_connection() -> Result<Db, Box<dyn std::error::Error + Sync + Send + 'static>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;

    Ok(Arc::new(
        PgPoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect(&database_url)
            .await?,
    ))
}
