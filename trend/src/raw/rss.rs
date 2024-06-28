use std::fmt::Display;

use date::Date;

use super::{CollectedRawTrends, RawTrendCollector, RawTrendInfo, RawTrendInfoError, Service};

pub struct RssRawTrendCollector<B: AsRef<[u8]>> {
    service: Service,
    bytes: B,
}
impl<B: AsRef<[u8]>> RssRawTrendCollector<B> {
    pub fn new(service: Service, bytes: B) -> Self {
        Self { service, bytes }
    }
    async fn to_channel(&self) -> Result<rss::Channel, RssRawTrendCollectorError> {
        let bytes = self.bytes.as_ref();
        rss::Channel::read_from(bytes).map_err(|e| RssRawTrendCollectorError::RssError(e))
    }
}
#[derive(Debug)]
pub enum RssRawTrendCollectorError {
    RssError(rss::Error),
}
impl std::fmt::Display for RssRawTrendCollectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RssRawTrendCollectorError::RssError(e) => write!(f, "RssError: {:?}", e),
        }
    }
}
impl std::error::Error for RssRawTrendCollectorError {}

impl<B: AsRef<[u8]>> RawTrendCollector for RssRawTrendCollector<B> {
    type Error = RssRawTrendCollectorError;
    async fn collect(&self) -> Result<CollectedRawTrends, RssRawTrendCollectorError> {
        let channel = self.to_channel().await?;
        Ok(CollectedRawTrends::new(
            channel
                .items()
                .iter()
                .filter_map(|item| item_to_trend(item, self.service.clone()).ok())
                .collect(),
        ))
    }
}

fn item_to_trend(item: &rss::Item, from: Service) -> Result<RawTrendInfo, RawTrendInfoError> {
    const DATE_FORMAT: &'static str = "%a, %d %b %Y %H:%M:%S %z";
    let title = item.title().unwrap_or_default().to_string();
    let link = item.link().unwrap_or_default().to_string();
    let desc = item.description().unwrap_or_default().to_string();
    let pub_date = item.pub_date().unwrap_or_default();
    let created_at = Date::parse_from_str(pub_date, DATE_FORMAT)
        .map_err(|_| RawTrendInfoError::InvalidDate(pub_date.to_string()))?;
    Ok(RawTrendInfo::new(title, link, desc, from, created_at))
}

pub struct RemoteRssRawTrendCollector {
    url: &'static str,
    service: Service,
}
impl RemoteRssRawTrendCollector {
    pub fn new(url: &'static str, service: Service) -> Self {
        Self { url, service }
    }
    pub fn aws_updates() -> Self {
        Self::new(
            "https://aws.amazon.com/jp/about-aws/whats-new/recent/feed/",
            Service::aws_updates(),
        )
    }
}

#[derive(Debug)]
pub enum RemoteRssRawTrendCollectorError {
    RequestError(reqwest::Error),
    RssError(RssRawTrendCollectorError),
}
impl Display for RemoteRssRawTrendCollectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RemoteRssRawTrendCollectorError::RequestError(e) => write!(f, "RequestError: {:?}", e),
            RemoteRssRawTrendCollectorError::RssError(e) => write!(f, "RssError: {:?}", e),
        }
    }
}
impl std::error::Error for RemoteRssRawTrendCollectorError {}

impl RawTrendCollector for RemoteRssRawTrendCollector {
    type Error = RemoteRssRawTrendCollectorError;
    async fn collect(&self) -> Result<CollectedRawTrends, Self::Error> {
        let bytes = reqwest::get(self.url)
            .await
            .map_err(|e| RemoteRssRawTrendCollectorError::RequestError(e))?
            .bytes()
            .await
            .map_err(|e| RemoteRssRawTrendCollectorError::RequestError(e))?;
        let collector = RssRawTrendCollector::new(self.service.clone(), bytes);
        collector
            .collect()
            .await
            .map_err(|e| RemoteRssRawTrendCollectorError::RssError(e))
    }
}

#[cfg(test)]
mod tests {
    use tokio::io::AsyncReadExt;

    use super::*;

    // Example 1 is written bottom of the items, but this item earlier than others.
    const DUMMY: &'static str = r#"
<?xml version="1.0" encoding="UTF-8" ?>
<rss version="2.0">
  <channel>
    <title>Example RSS Feed</title>
    <link>http://www.example.com</link>
    <description>This is an example of an RSS feed</description>
    <lastBuildDate>Fri, 21 Jun 2024 02:22:32 +0000</lastBuildDate>
    <pubDate>Fri, 21 Jun 2024 02:22:32 +0000</pubDate>
    <ttl>1800</ttl>

    <item>
      <title>Example Item 2</title>
      <link>http://www.example.com/item2</link>
      <description>This is the description for example item 2.</description>
      <author>author@example.com</author>
      <pubDate>Fri, 21 Jun 2024 02:22:32 +0000</pubDate>
      <guid>http://www.example.com/item2</guid>
    </item>

    <item>
      <title>Example Item 3</title>
      <link>http://www.example.com/item3</link>
      <description>This is the description for example item 3.</description>
      <author>author@example.com</author>
      <pubDate>Fri, 21 Jun 2024 02:22:32 +0000</pubDate>
      <guid>http://www.example.com/item3</guid>
    </item>

    <item>
      <title>Example Item 1</title>
      <link>http://www.example.com/item1</link>
      <description>This is the description for example item 1.</description>
      <author>author@example.com</author>
      <pubDate>Sat, 22 Jun 2024 02:22:32 +0000</pubDate>
      <guid>http://www.example.com/item1</guid>
    </item>

  </channel>
</rss>
"#;
    #[tokio::test]
    async fn collect_all_rss_item() {
        let collector =
            RssRawTrendCollector::new(Service::aws_updates(), DUMMY.as_bytes().to_vec());
        let infos = collector.collect().await.unwrap();

        assert_eq!(infos.trends().len(), 3);
    }
    #[tokio::test]
    async fn collect_rss_to_trend_should_sorted_by_pub_date() {
        let collector =
            RssRawTrendCollector::new(Service::aws_updates(), DUMMY.as_bytes().to_vec());
        let infos = collector.collect().await.unwrap();

        assert_eq!(infos.latest().unwrap().title(), "Example Item 1");
    }
    #[tokio::test]
    async fn collect_rss_to_trend_has_from_field() {
        let collector =
            RssRawTrendCollector::new(Service::aws_updates(), DUMMY.as_bytes().to_vec());
        let infos = collector.collect().await.unwrap();

        assert_eq!(
            infos.latest().unwrap().from(),
            Service::aws_updates().to_str()
        );
    }
    #[tokio::test]
    async fn collect_aws_rss_to_trend() {
        let mut reader =
            tokio::io::BufReader::new(tokio::fs::File::open("../tests/aws.rss").await.unwrap());
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await.unwrap();

        let collector = RssRawTrendCollector::new(Service::aws_updates(), buf);
        let infos = collector.collect().await.unwrap();

        assert_eq!(infos.latest().unwrap().title, "hogehoge")
    }
}
