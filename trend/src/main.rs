use std::env;

use axum::{routing::get, Json, Router};
use trend::raw::{rss::RemoteRssRawTrendCollector, RawTrendCollector, Trend};

async fn new() -> Json<Vec<Trend>> {
    println!("called new");
    let aws = RemoteRssRawTrendCollector::aws_updates();
    let infos = aws.collect().await.unwrap();
    Json(<Vec<Trend>>::from(infos))
}

async fn health_check() -> &'static str {
    println!("called health_check");
    "ok"
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let app = Router::new()
        .route("/new", get(new))
        .route("/health_check", get(health_check));

    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
