// trend life cycle
// Collect Raw Trend Info ->User can see New Raw Trend Info and have been checked Trend Infos -> User can check Raw Trend Info -> User can update check status
use date::Date;

use crate::raw::{RawTrendInfo, Service};

//pub trait UserTrendInfoRepository {
//    async fn save(&self, user_id: UserId) -> Result<UserTrendInfo, UserTrendInfoRepositoryError>;
//}
#[derive(Debug)]
pub enum UserTrendInfoRepositoryError {
    SaveError(String),
}

#[derive(Debug, Clone)]
pub struct TrendInfoEntity {
    id: String,
    user_id: String,
    link: String,
    title: String,
    desc: String,
    memo: String,
    from: Service,
    status: String,
    created_at: String,
    updated_at: String,
}

//#[cfg(test)]
//pub mod fake {
//    use std::cell::RefCell;
//
//    use super::{TrendInfo, UserTrendInfoRepository};
//
//    pub struct FakeUserTrendInfoRepository {
//        infos: RefCell<Vec<TrendInfo>>,
//    }
//    impl UserTrendInfoRepository for FakeUserTrendInfoRepository {
//        async fn save(
//            &self,
//            user_id: UserId,
//            info: crate::raw::RawTrendInfo,
//        ) -> Result<TrendInfo, super::UserTrendInfoRepositoryError> {
//            if self.infos.borrow().iter().any(|i| i.link == info.link()) {
//                return Err(super::UserTrendInfoRepositoryError::SaveError(
//                    "already saved".to_string(),
//                ));
//            }
//            let result = TrendInfo {
//                id: super::TrendInfoId("id".to_string()),
//                user_id,
//                link: info.link.clone(),
//                title: info.title.clone(),
//                desc: info.desc.clone(),
//                memo: "".to_string(),
//                from: info.from.clone(),
//                status: super::Status::New,
//                created_at: info.created_at.clone(),
//                updated_at: info.created_at.clone(),
//            };
//            self.infos.borrow_mut().push(result.clone());
//            Ok(result)
//        }
//    }
//}
