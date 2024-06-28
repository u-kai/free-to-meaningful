pub mod domain;
pub mod raw;
pub mod repository;

//pub struct TrendService<R: UserTrendInfoRepository> {
//    repository: R,
//}

#[cfg(test)]
mod tests {
    use crate::raw::{RawTrendInfo, Service};
    //use crate::repository::Status;
    use date::Date;

    use super::*;
    //#[tokio::test]
    //async fn user_can_save_new_trend_with_other_info() {
    //    let user_id = UserId::new("user_id");
    //    let title = "title";
    //    let link = "link";
    //    let raw_trend = RawTrendInfo::new(
    //        title,
    //        link,
    //        "desc",
    //        Service::aws_updates(),
    //        Date::parse_from_str("2021-01-01", "%Y-%m-%d").unwrap(),
    //    );
    //    let memo = "so, interesting!";
    //    let status = Status::ToDo;
    //    let save_info = SaveUserTrendInfoBuilder::new(user_id)
    //        .memo(memo)
    //        .status(status)
    //        .build();

    //    let service = TrendService::new(FakeUserTrendInfoRepository::new());

    //    let trend = service.save(raw_trend, save_info).await.unwrap();

    //    assert_eq!(trend.title(), title);
    //    assert_eq!(trend.link(), link);
    //    assert_eq!(trend.memo(), memo);
    //    assert_eq!(trend.status(), status);
    //}
}
