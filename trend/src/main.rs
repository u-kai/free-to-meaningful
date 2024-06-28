use std::env;

use axum::{routing::get, Json, Router};
use trend::raw::{rss::RemoteRssRawTrendCollector, RawTrendCollector, Trend};

async fn new() -> Json<Vec<Trend>> {
    let aws = RemoteRssRawTrendCollector::aws_updates();
    let infos = aws.collect().await.unwrap();
    Json(<Vec<Trend>>::from(infos))
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("127.0.0.1:{}", port);
    let app = Router::new().route("/new", get(new));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
