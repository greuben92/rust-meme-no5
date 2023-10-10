use axum::extract::State;
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    let router = axum::Router::new()
        .route("/", axum::routing::get(bug))
        .with_state(pool);
    let addr: SocketAddr = "127.0.0.1:6969".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn bug(State(pool): State<SqlitePool>) -> &'static str {
    let contents = reqwest::get("https://www.google.com/")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    // The next line somehow causes the bug
    let _doc = scraper::Html::parse_document(&contents);
    let tx = pool.begin().await.unwrap();
    tx.commit().await.unwrap();
    "uwu"
}
