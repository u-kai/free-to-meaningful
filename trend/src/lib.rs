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

    #[test]
    fn collect_trend_should_sorted_pub_date() {}
    #[test]
    fn collect_rss_to_trend() {}
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
