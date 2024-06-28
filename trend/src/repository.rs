// trend life cycle
// Collect Raw Trend Info ->User can see New Raw Trend Info and have been checked Trend Infos -> User can check Raw Trend Info -> User can update check status

use std::fmt::Display;

use date::Date;

use crate::{
    domain::{Status, UserTrendInfo, UserTrendInfoId},
    raw::{RawTrendInfo, Service},
    use_case::SaveNewTrendInfo,
};

#[derive(Debug, Clone)]
pub struct InitTrendInfoEntity {
    pub user_id: String,
    pub link: String,
    pub title: String,
    pub desc: String,
    pub memo: String,
    pub from: String,
    pub status: String,
    pub created_at: String,
}
impl InitTrendInfoEntity {
    pub fn new(user_trend: SaveNewTrendInfo) -> Self {
        Self {
            user_id: user_trend.user_id.to_string(),
            link: user_trend.raw_trend.link,
            title: user_trend.raw_trend.title,
            desc: user_trend.raw_trend.desc,
            memo: user_trend.memo,
            from: user_trend.raw_trend.from.to_str().to_string(),
            status: user_trend.status.to_str().to_string(),
            created_at: user_trend.raw_trend.created_at.to_string(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TrendInfoEntity {
    pub id: String,
    pub user_id: String,
    pub link: String,
    pub title: String,
    pub desc: String,
    pub memo: String,
    pub from: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}
impl TryInto<UserTrendInfo> for TrendInfoEntity {
    type Error = TrendInfoEntityError;
    // Even if it fails, the entity is already saved in the DB, so what should be done in such cases?
    // I feel like this code lacks elegance
    fn try_into(self) -> Result<UserTrendInfo, Self::Error> {
        let id = UserTrendInfoId(self.id);
        let created_at = self.created_at;
        let raw_info = RawTrendInfo::new(
            self.title,
            self.link,
            self.desc,
            Service::from(self.from),
            Date::from_str(&created_at)
                .map_err(|_| TrendInfoEntityError::InvalidDate(created_at))?,
        );
        let mut result = UserTrendInfo::new(id, raw_info);
        // TODO: remove unwrap
        result.change_memo(self.memo).unwrap();
        let new_status = Status::from_str(self.status.as_str())
            .map_err(|_| TrendInfoEntityError::InvalidStatus(self.status.clone()))?;
        result
            .change_status(new_status)
            .map_err(|e| TrendInfoEntityError::InvalidStatusChange(e.to_string()))?;
        Ok(result)
    }
}
#[derive(Debug)]
pub enum TrendInfoEntityError {
    InvalidStatus(String),
    InvalidDate(String),
    InvalidStatusChange(String),
}
impl Display for TrendInfoEntityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrendInfoEntityError::InvalidStatus(s) => write!(f, "InvalidStatus: {}", s),
            TrendInfoEntityError::InvalidDate(s) => write!(f, "InvalidDate: {}", s),
            TrendInfoEntityError::InvalidStatusChange(s) => write!(f, "InvalidStatusChange: {}", s),
        }
    }
}
impl std::error::Error for TrendInfoEntityError {}

#[cfg(test)]
pub mod fake {
    use std::cell::RefCell;

    use date::Date;

    use crate::{
        domain::UserTrendInfo,
        use_case::{SaveNewTrendInfo, UserTrendInfoRepository, UserTrendInfoRepositoryError},
    };

    use super::{InitTrendInfoEntity, TrendInfoEntity, TrendInfoEntityError};

    pub struct FakeUserTrendInfoRepository {
        infos: RefCell<Vec<TrendInfoEntity>>,
    }
    impl FakeUserTrendInfoRepository {
        pub fn new() -> Self {
            Self {
                infos: RefCell::new(vec![]),
            }
        }
    }
    impl UserTrendInfoRepository for FakeUserTrendInfoRepository {
        async fn update(
            &self,
            user_trend: UserTrendInfo,
        ) -> Result<UserTrendInfo, UserTrendInfoRepositoryError> {
            let mut infos = self.infos.borrow_mut();
            let entity = infos.iter_mut().find(|i| i.id == user_trend.id().0).ok_or(
                UserTrendInfoRepositoryError::NotFoundError("not found".to_string()),
            )?;
            entity.memo = user_trend.memo().to_string();
            entity.status = user_trend.status().to_str().to_string();
            entity.updated_at = Date::now().to_string();
            Ok(user_trend)
        }
        async fn list(
            &self,
            user_id: user::UserId,
        ) -> Result<Vec<UserTrendInfo>, UserTrendInfoRepositoryError> {
            let infos = self.infos.borrow();
            let result: Result<Vec<UserTrendInfo>, TrendInfoEntityError> = infos
                .iter()
                .filter(|i| i.user_id == user_id.to_string())
                .map(|i| i.clone().try_into())
                .collect();
            Ok(result.map_err(|e| UserTrendInfoRepositoryError::ConvertError(e.to_string()))?)
        }
        async fn save(
            &self,
            user_trend: SaveNewTrendInfo,
        ) -> Result<UserTrendInfo, UserTrendInfoRepositoryError> {
            if self.infos.borrow().iter().any(|i| {
                i.link == user_trend.raw_trend.link() && user_trend.user_id.is_same(&i.user_id)
            }) {
                return Err(UserTrendInfoRepositoryError::AlreadyExists(
                    "already saved".to_string(),
                ));
            }
            let entity = InitTrendInfoEntity::new(user_trend.clone());
            // fake to save in db
            let entity = TrendInfoEntity {
                id: format!("id-{}", self.infos.borrow().len()),
                user_id: entity.user_id,
                link: entity.link,
                title: entity.title,
                desc: entity.desc,
                memo: entity.memo,
                from: entity.from,
                status: entity.status,
                created_at: entity.created_at,
                updated_at: Date::now().to_string(),
            };
            self.infos.borrow_mut().push(entity.clone());

            let result: Result<UserTrendInfo, TrendInfoEntityError> = entity.try_into();
            result.map_err(|e| UserTrendInfoRepositoryError::ConvertError(e.to_string()))
        }
    }
}
