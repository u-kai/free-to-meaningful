use trend::{remote::RemoteRssTrendCollector, TrendCollector};

#[tokio::main]
async fn main() {
    let aws = RemoteRssTrendCollector::aws_updates();
    let infos = aws.collect().await.unwrap();
    for trend in infos.trends() {
        println!("title: {}", trend.title());
        println!("link : {}", trend.link());
    }
}
