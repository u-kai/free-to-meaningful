use date::Date;

pub mod rss;

pub trait RawTrendCollector {
    type Error: std::error::Error;
    #[allow(async_fn_in_trait)]
    async fn collect(&self) -> Result<CollectedRawTrends, Self::Error>;
}
#[derive(Debug)]
pub struct CollectedRawTrends {
    inner: Vec<RawTrendInfo>,
}
impl CollectedRawTrends {
    pub fn latest(&self) -> Option<&RawTrendInfo> {
        self.inner.first()
    }
    pub fn trends(&self) -> &[RawTrendInfo] {
        &self.inner
    }
    fn new(inner: Vec<RawTrendInfo>) -> Self {
        let inner = Self::sort(inner);
        Self { inner }
    }
    fn sort(mut inner: Vec<RawTrendInfo>) -> Vec<RawTrendInfo> {
        // Sort by pub_date desc
        // this mean is latest trend is first.
        inner.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        inner
    }
}

#[derive(Debug, Clone)]
pub struct RawTrendInfo {
    title: String,
    link: String,
    desc: String,
    created_at: Date,
}
impl RawTrendInfo {
    pub fn new(
        title: impl Into<String>,
        link: impl Into<String>,
        desc: impl Into<String>,
        created_at: Date,
    ) -> Self {
        Self {
            title: title.into(),
            link: link.into(),
            desc: desc.into(),
            created_at,
        }
    }
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

pub enum RawTrendInfoError {
    InvalidDate(String),
}
