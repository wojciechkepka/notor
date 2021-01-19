#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    warp::serve(notor::routes(notor::db::db_connection()))
        .run(([127, 0, 0, 1], 3693))
        .await;
}
