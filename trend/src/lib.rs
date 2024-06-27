use date::Date;
pub struct CollectedTrends {
    inner: Vec<TrendInfo>,
}
impl CollectedTrends {
    pub fn new(inner: Vec<TrendInfo>) -> Self {
        Self { inner }
    }
    pub fn latest(&self) -> Option<&TrendInfo> {
        self.inner.first()
    }
    pub fn trends(&self) -> &[TrendInfo] {
        &self.inner
    }
}
pub trait TrendCollector {
    #[allow(async_fn_in_trait)]
    async fn collect(&self) -> Result<CollectedTrends, TrendCollectorError>;
}
#[derive(Debug)]
pub struct TrendCollectorError;

pub enum TrendInfoError {
    InvalidDate(String),
}
#[derive(Debug)]
pub struct TrendInfo {
    title: String,
    link: String,
    desc: String,
    created_at: Date,
}
impl TrendInfo {
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn link(&self) -> &str {
        &self.link
    }
    pub fn desc(&self) -> &str {
        &self.desc
    }
    pub fn created_at(&self) -> &Date {
        &self.created_at
    }
}

pub struct RssTrendCollector {
    bytes: Vec<u8>,
}
impl RssTrendCollector {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
    async fn to_channel(&self) -> Result<rss::Channel, RssTrendCollectorError> {
        rss::Channel::read_from(&self.bytes[..]).map_err(|e| RssTrendCollectorError::RssError(e))
    }
}
pub enum RssTrendCollectorError {
    RssError(rss::Error),
}
impl TrendCollector for RssTrendCollector {
    async fn collect(&self) -> Result<CollectedTrends, TrendCollectorError> {
        let channel = self.to_channel().await.map_err(|_| TrendCollectorError)?;
        Ok(CollectedTrends::new(
            channel
                .items()
                .iter()
                .filter_map(|item| item_to_trend(item).ok())
                .collect(),
        ))
    }
}

fn item_to_trend(item: &rss::Item) -> Result<TrendInfo, TrendInfoError> {
    const DATE_FORMAT: &'static str = "%a, %d %b %Y %H:%M:%S %z";
    let title = item.title().unwrap_or_default().to_string();
    let link = item.link().unwrap_or_default().to_string();
    let desc = item.description().unwrap_or_default().to_string();
    let pub_date = item.pub_date().unwrap_or_default();
    let created_at = Date::parse_from_str(pub_date, DATE_FORMAT)
        .map_err(|_| TrendInfoError::InvalidDate(pub_date.to_string()))?;
    Ok(TrendInfo {
        title,
        link,
        desc,
        created_at,
    })
}

#[cfg(test)]
mod tests {
    use tokio::io::AsyncReadExt;

    use super::*;

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
      <title>Example Item 1</title>
      <link>http://www.example.com/item1</link>
      <description>This is the description for example item 1.</description>
      <author>author@example.com</author>
      <pubDate>Fri, 21 Jun 2024 02:22:32 +0000</pubDate>
      <guid>http://www.example.com/item1</guid>
    </item>

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

  </channel>
</rss>

"#;
    #[test]
    fn collect_trend_should_sorted_pub_date() {}
    #[tokio::test]
    async fn collect_rss_to_trend() {
        let collector = RssTrendCollector::new(DUMMY.as_bytes().to_vec());
        let infos = collector.collect().await.unwrap();

        assert_eq!(infos.latest().unwrap().title(), "Example Item 1");
        assert_eq!(infos.trends().len(), 3);
    }
    #[tokio::test]
    async fn collect_rss_to_aws_trend() {
        let mut reader =
            tokio::io::BufReader::new(tokio::fs::File::open("../tests/aws.rss").await.unwrap());
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await.unwrap();

        let collector = RssTrendCollector::new(buf);
        let infos = collector.collect().await.unwrap();

        assert_eq!(infos.latest().unwrap().title, "hogehoge")
    }
}
