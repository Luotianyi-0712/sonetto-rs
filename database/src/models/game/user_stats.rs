use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct UserStats {
    pub user_id: i64,
    pub first_charge: bool,
    pub total_charge_amount: i64,
    pub is_first_login: bool,
    pub user_tag: String,
}

impl From<UserStats> for sonettobuf::StatInfoPush {
    fn from(stats: UserStats) -> Self {
        sonettobuf::StatInfoPush {
            frist_charge: Some(stats.first_charge),
            total_charge_amount: Some(stats.total_charge_amount),
            is_first_login: Some(stats.is_first_login),
            player_info: None, // game has this null for some reason
            user_tag: Some(stats.user_tag),
        }
    }
}
