use date::Date;
pub mod remote;
#[derive(Debug)]
pub struct CollectedTrends {
    inner: Vec<TrendInfo>,
}
impl CollectedTrends {
    pub fn new(inner: Vec<TrendInfo>) -> Self {
        let inner = Self::sort(inner);
        Self { inner }
    }
    pub fn latest(&self) -> Option<&TrendInfo> {
        self.inner.first()
    }
    pub fn trends(&self) -> &[TrendInfo] {
        &self.inner
    }
    fn sort(mut inner: Vec<TrendInfo>) -> Vec<TrendInfo> {
        // Sort by pub_date desc
        // this mean is latest trend is first.
        inner.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        inner
    }
}
pub trait TrendCollector {
    type Error: std::error::Error;
    #[allow(async_fn_in_trait)]
    async fn collect(&self) -> Result<CollectedTrends, Self::Error>;
}

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

pub struct RssTrendCollector<B: AsRef<[u8]>> {
    bytes: B,
}
impl<B: AsRef<[u8]>> RssTrendCollector<B> {
    pub fn new(bytes: B) -> Self {
        Self { bytes }
    }
    async fn to_channel(&self) -> Result<rss::Channel, RssTrendCollectorError> {
        let bytes = self.bytes.as_ref();
        rss::Channel::read_from(bytes).map_err(|e| RssTrendCollectorError::RssError(e))
    }
}
#[derive(Debug)]
pub enum RssTrendCollectorError {
    RssError(rss::Error),
}
impl std::fmt::Display for RssTrendCollectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RssTrendCollectorError::RssError(e) => write!(f, "RssError: {:?}", e),
        }
    }
}
impl std::error::Error for RssTrendCollectorError {}

impl<B: AsRef<[u8]>> TrendCollector for RssTrendCollector<B> {
    type Error = RssTrendCollectorError;
    async fn collect(&self) -> Result<CollectedTrends, RssTrendCollectorError> {
        let channel = self.to_channel().await?;
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
        let collector = RssTrendCollector::new(DUMMY.as_bytes().to_vec());
        let infos = collector.collect().await.unwrap();

        assert_eq!(infos.trends().len(), 3);
    }
    #[tokio::test]
    async fn collect_rss_to_trend_should_sorted_by_pub_date() {
        let collector = RssTrendCollector::new(DUMMY.as_bytes().to_vec());
        let infos = collector.collect().await.unwrap();

        assert_eq!(infos.latest().unwrap().title(), "Example Item 1");
    }
    #[tokio::test]
    async fn collect_aws_rss_to_trend() {
        let mut reader =
            tokio::io::BufReader::new(tokio::fs::File::open("../tests/aws.rss").await.unwrap());
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await.unwrap();

        let collector = RssTrendCollector::new(buf);
        let infos = collector.collect().await.unwrap();

        assert_eq!(infos.latest().unwrap().title, "hogehoge")
    }
}
