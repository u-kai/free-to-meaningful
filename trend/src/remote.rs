use std::fmt::Display;

use crate::{CollectedTrends, RssTrendCollector, RssTrendCollectorError, TrendCollector};

pub struct RemoteRssTrendCollector {
    url: &'static str,
}
impl RemoteRssTrendCollector {
    pub fn new(url: &'static str) -> Self {
        Self { url }
    }
    pub fn aws_updates() -> Self {
        Self::new("https://aws.amazon.com/jp/about-aws/whats-new/recent/feed/")
    }
}

#[derive(Debug)]
pub enum RemoteRssTrendCollectorError {
    RequestError(reqwest::Error),
    RssError(RssTrendCollectorError),
}
impl Display for RemoteRssTrendCollectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RemoteRssTrendCollectorError::RequestError(e) => write!(f, "RequestError: {:?}", e),
            RemoteRssTrendCollectorError::RssError(e) => write!(f, "RssError: {:?}", e),
        }
    }
}
impl std::error::Error for RemoteRssTrendCollectorError {}

impl TrendCollector for RemoteRssTrendCollector {
    type Error = RemoteRssTrendCollectorError;
    async fn collect(&self) -> Result<CollectedTrends, Self::Error> {
        let bytes = reqwest::get(self.url)
            .await
            .map_err(|e| RemoteRssTrendCollectorError::RequestError(e))?
            .bytes()
            .await
            .map_err(|e| RemoteRssTrendCollectorError::RequestError(e))?;
        let collector = RssTrendCollector::new(bytes);
        collector
            .collect()
            .await
            .map_err(|e| RemoteRssTrendCollectorError::RssError(e))
    }
}
