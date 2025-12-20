use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct UserSignInInfo {
    pub user_id: i64,
    pub addup_sign_in_day: i32,
    pub open_function_time: i32,
    pub reward_mark: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct MonthCardHistory {
    pub card_id: i32,
    pub start_time: i32,
    pub end_time: i32,
}

impl From<MonthCardHistory> for sonettobuf::MonthCardHistory {
    fn from(m: MonthCardHistory) -> Self {
        sonettobuf::MonthCardHistory {
            id: Some(m.card_id),
            start_time: Some(m.start_time),
            end_time: Some(m.end_time),
        }
    }
}
