#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(String);

impl UserId {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
    pub fn is_same(&self, other: &str) -> bool {
        self.0 == other
    }
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
