use trend::raw::{rss::RemoteRssRawTrendCollector, RawTrendCollector};

#[tokio::main]
async fn main() {
    let aws = RemoteRssRawTrendCollector::aws_updates();
    let infos = aws.collect().await.unwrap();
    for trend in infos.trends() {
        println!("title: {}", trend.title());
        println!("link : {}", trend.link());
    }
}
