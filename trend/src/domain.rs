use crate::raw::RawTrendInfo;

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
    pub fn memo(&self) -> &str {
        &self.memo.0
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
    InvalidMemo(MemoError),
    InvalidStatus(StatusError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserTrendInfoId(String);

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

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Status {
    New,
    Reading,
    ToDo,
    Done,
}
impl Status {
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
}
#[derive(Debug)]
enum StatusError {
    InvalidStatusChange(Status, Status),
}

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