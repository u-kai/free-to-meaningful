use date::Date;
pub mod rss;
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
