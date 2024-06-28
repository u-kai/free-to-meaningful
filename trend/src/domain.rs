use date::Date;

use crate::raw::RawTrendInfo;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct UserTrendInfo {
    id: UserTrendInfoId,
    raw_info: RawTrendInfo,
    memo: Memo,
    status: Status,
}
impl UserTrendInfo {
    pub fn new(id: UserTrendInfoId, raw_info: RawTrendInfo) -> Self {
        Self {
            id,
            raw_info,
            memo: Memo::new(),
            status: Status::New,
        }
    }
    pub fn id(&self) -> &UserTrendInfoId {
        &self.id
    }
    pub fn raw_info(&self) -> &RawTrendInfo {
        &self.raw_info
    }
    pub fn memo(&self) -> &str {
        &self.memo.0
    }
    pub fn title(&self) -> &str {
        self.raw_info.title()
    }
    pub fn link(&self) -> &str {
        self.raw_info.link()
    }
    pub fn from(&self) -> &str {
        self.raw_info.from()
    }
    pub fn created_at(&self) -> &Date {
        self.raw_info.created_at()
    }
    pub fn change_status(&mut self, new_status: Status) -> Result<(), UserTrendInfoError> {
        self.status = self
            .status
            .change_status(new_status)
            .map(|s| s)
            .map_err(|e| UserTrendInfoError::InvalidStatus(e))?;
        Ok(())
    }
    pub fn status(&self) -> Status {
        self.status
    }
    pub fn change_memo(&mut self, new_memo: String) -> Result<(), UserTrendInfoError> {
        self.memo
            .change_memo(new_memo)
            .map_err(|e| UserTrendInfoError::InvalidMemo(e))
    }
}

#[derive(Debug)]
pub enum UserTrendInfoError {
    #[allow(private_interfaces)]
    InvalidMemo(MemoError),
    #[allow(private_interfaces)]
    InvalidStatus(StatusError),
}
impl Display for UserTrendInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserTrendInfoError::InvalidMemo(e) => write!(f, "InvalidMemo: {}", e.to_string()),
            UserTrendInfoError::InvalidStatus(e) => write!(f, "InvalidStatus: {}", e.to_string()),
        }
    }
}
impl std::error::Error for UserTrendInfoError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserTrendInfoId(pub(super) String);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Memo(String);

impl Memo {
    const MAX_LEN: usize = 500;
    fn new() -> Self {
        Self(String::new())
    }
    fn change_memo(&mut self, new_memo: String) -> Result<(), MemoError> {
        if new_memo.len() > Self::MAX_LEN {
            return Err(MemoError::TooLong(new_memo.len()));
        }
        self.0 = new_memo;
        Ok(())
    }
}
#[derive(Debug)]
enum MemoError {
    TooLong(usize),
}
impl Display for MemoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoError::TooLong(len) => write!(f, "too long: {}", len),
        }
    }
}
impl std::error::Error for MemoError {}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Status {
    New,
    Reading,
    ToDo,
    Done,
}
impl Status {
    pub fn from_str(s: &str) -> Result<Self, StatusError> {
        match s {
            "New" => Ok(Status::New),
            "Reading" => Ok(Status::Reading),
            "ToDo" => Ok(Status::ToDo),
            "Done" => Ok(Status::Done),
            _ => Err(StatusError::InvalidStatus(s.to_string())),
        }
    }
    fn change_status(self, to: Self) -> Result<Self, StatusError> {
        match (self, to) {
            (old, new) if old == new => Ok(old),
            // New to other is always allowed
            (Status::New, s) => Ok(s),
            // Some status to New is always denied
            (s, Status::New) => Err(StatusError::InvalidStatusChange(s, Status::New)),
            _ => Ok(to),
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            Status::New => "New",
            Status::Reading => "Reading",
            Status::ToDo => "ToDo",
            Status::Done => "Done",
        }
    }
}
#[derive(Debug)]
pub enum StatusError {
    InvalidStatusChange(Status, Status),
    InvalidStatus(String),
}
impl Display for StatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusError::InvalidStatusChange(from, to) => {
                write!(
                    f,
                    "InvalidStatusChange: {} -> {}",
                    from.to_str(),
                    to.to_str()
                )
            }
            StatusError::InvalidStatus(s) => write!(f, "InvalidStatus: {}", s),
        }
    }
}

impl std::error::Error for StatusError {}

#[cfg(test)]
mod tests {
    use date::Date;

    use crate::{
        domain::{Status, UserTrendInfo, UserTrendInfoId},
        raw::{RawTrendInfo, Service},
    };

    #[test]
    fn user_trend_info_can_change_memo() {
        let id = UserTrendInfoId("id".to_string());
        let raw_info =
            RawTrendInfo::new("title", "link", "desc", Service::aws_updates(), Date::now());
        let mut info = UserTrendInfo::new(id, raw_info);
        let new_memo = "new memo";

        info.change_memo(new_memo.to_string()).unwrap();

        assert_eq!(info.memo(), new_memo);
    }
    #[test]
    fn user_trend_info_can_not_change_big_memo() {
        let id = UserTrendInfoId("id".to_string());
        let raw_info =
            RawTrendInfo::new("title", "link", "desc", Service::aws_updates(), Date::now());
        let mut info = UserTrendInfo::new(id, raw_info);
        let new_big_memo = "a".repeat(10000);

        let result = info.change_memo(new_big_memo);

        assert!(result.is_err());
    }
    #[test]
    fn user_trend_info_can_change_status() {
        let id = UserTrendInfoId("id".to_string());
        let raw_info =
            RawTrendInfo::new("title", "link", "desc", Service::aws_updates(), Date::now());
        let mut info = UserTrendInfo::new(id, raw_info);
        let new_status = Status::Reading;

        info.change_status(new_status).unwrap();

        assert_eq!(info.status, new_status);
    }
}
