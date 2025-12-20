use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct UserAntique {
    pub user_id: i64,
    pub antique_id: i32,
    pub get_time: i64,
}

impl From<UserAntique> for sonettobuf::AntiqueInfo {
    fn from(a: UserAntique) -> Self {
        sonettobuf::AntiqueInfo {
            antique_id: Some(a.antique_id),
            get_time: Some(a.get_time as u64),
        }
    }
}
