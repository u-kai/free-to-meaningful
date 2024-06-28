use user::UserId;

use crate::{
    domain::{Status, UserTrendInfo},
    raw::RawTrendInfo,
};

pub trait UserTrendInfoRepository {
    #[allow(async_fn_in_trait)]
    async fn save(
        &self,
        user_trend: SaveNewTrendInfo,
    ) -> Result<UserTrendInfo, UserTrendInfoRepositoryError>;
    #[allow(async_fn_in_trait)]
    async fn update(
        &self,
        user_trend: UserTrendInfo,
    ) -> Result<UserTrendInfo, UserTrendInfoRepositoryError>;
    #[allow(async_fn_in_trait)]
    async fn list(
        &self,
        user_id: UserId,
    ) -> Result<Vec<UserTrendInfo>, UserTrendInfoRepositoryError>;
}

#[derive(Debug)]
pub enum UserTrendInfoRepositoryError {
    SaveError(String),
    AlreadyExists(String),
    ConvertError(String),
    NotFoundError(String),
}

pub async fn save_new_trend(
    repository: &impl UserTrendInfoRepository,
    save_info: SaveNewTrendInfo,
) -> Result<UserTrendInfo, UserTrendInfoRepositoryError> {
    repository.save(save_info).await
}

pub async fn update_trend(
    repository: &impl UserTrendInfoRepository,
    user_trend: UserTrendInfo,
) -> Result<UserTrendInfo, UserTrendInfoRepositoryError> {
    repository.update(user_trend).await
}

#[derive(Debug, Clone)]
pub struct SaveNewTrendInfo {
    pub(super) user_id: UserId,
    pub(super) raw_trend: RawTrendInfo,
    pub(super) memo: String,
    pub(super) status: Status,
}
pub struct SaveNewTrendInfoBuilder {
    user_id: UserId,
    raw_trend: RawTrendInfo,
    memo: String,
    status: Status,
}
impl SaveNewTrendInfoBuilder {
    pub fn new(user_id: UserId, raw_trend: RawTrendInfo) -> Self {
        Self {
            user_id,
            raw_trend,
            memo: "".to_string(),
            status: Status::New,
        }
    }
    pub fn memo(mut self, memo: impl Into<String>) -> Self {
        self.memo = memo.into();
        self
    }
    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }
    pub fn build(self) -> SaveNewTrendInfo {
        SaveNewTrendInfo {
            user_id: self.user_id,
            raw_trend: self.raw_trend,
            memo: self.memo,
            status: self.status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::Status,
        raw::{RawTrendInfo, Service},
        repository::fake::FakeUserTrendInfoRepository,
    };
    use date::Date;
    use user::UserId;
    #[tokio::test]
    async fn user_can_update_trend() {
        let user_id = UserId::new("user_id");
        let title = "title";
        let link = "link";
        let raw_trend = RawTrendInfo::new(
            title,
            link,
            "desc",
            Service::aws_updates(),
            Date::parse_from_str("2021-01-01", "%Y-%m-%d").unwrap(),
        );
        let memo = "so, interesting!";
        let status = Status::ToDo;
        let save_info = SaveNewTrendInfoBuilder::new(user_id, raw_trend)
            .memo(memo)
            .status(status)
            .build();

        let repository = FakeUserTrendInfoRepository::new();

        let mut user_trend = save_new_trend(&repository, save_info).await.unwrap();
        user_trend.change_status(Status::Done).unwrap();

        let user_trend = update_trend(&repository, user_trend).await.unwrap();

        assert_eq!(user_trend.status(), Status::Done);
    }
    #[tokio::test]
    async fn user_can_save_new_trend_with_other_info() {
        let user_id = UserId::new("user_id");
        let title = "title";
        let link = "link";
        let raw_trend = RawTrendInfo::new(
            title,
            link,
            "desc",
            Service::aws_updates(),
            Date::parse_from_str("2021-01-01", "%Y-%m-%d").unwrap(),
        );
        let memo = "so, interesting!";
        let status = Status::ToDo;
        let save_info = SaveNewTrendInfoBuilder::new(user_id, raw_trend)
            .memo(memo)
            .status(status)
            .build();

        let repository = FakeUserTrendInfoRepository::new();

        let user_trend = save_new_trend(&repository, save_info).await.unwrap();

        assert_eq!(user_trend.title(), title);
        assert_eq!(user_trend.link(), link);
        assert_eq!(user_trend.memo(), memo);
        assert_eq!(user_trend.status(), status);
    }
}
