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

pub struct TrendInfoError;
#[derive(Debug)]
pub struct TrendInfo {
    title: String,
    link: String,
    desc: String,
    created_at: Date,
}

#[derive(Debug)]
// TODO: Implement Date
pub struct Date;

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
                .filter_map(|item| TrendInfo::try_from(item).ok())
                .collect(),
        ))
    }
}

impl TryFrom<&rss::Item> for TrendInfo {
    type Error = TrendInfoError;
    fn try_from(value: &rss::Item) -> Result<Self, Self::Error> {
        Ok(Self {
            title: value.title().unwrap_or_default().to_string(),
            link: value.link().unwrap_or_default().to_string(),
            desc: value.description().unwrap_or_default().to_string(),
            created_at: Date,
        })
    }
}

#[cfg(test)]
mod tests {
    use tokio::io::AsyncReadExt;

    use super::*;

    #[tokio::test]
    async fn collect_rss_to_trend() {
        let mut reader =
            tokio::io::BufReader::new(tokio::fs::File::open("../tests/aws.rss").await.unwrap());
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await.unwrap();

        let collector = RssTrendCollector::new(buf);
        let infos = collector.collect().await.unwrap();

        assert_eq!(infos.latest().unwrap().title, "hogehoge")
    }
}
