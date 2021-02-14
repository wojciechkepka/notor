#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    match notor::db::db_connection() {
        Ok(conn) => {
            warp::serve(notor::routes(conn))
                .run(([127, 0, 0, 1], 3693))
                .await;
        }
        Err(e) => eprintln!("{}", e),
    }
}
