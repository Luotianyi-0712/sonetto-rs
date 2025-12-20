use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct ChargeInfo {
    pub user_id: i64,
    pub charge_id: i32,
    pub buy_count: i32,
    pub first_charge: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<ChargeInfo> for sonettobuf::ChargeInfo {
    fn from(c: ChargeInfo) -> Self {
        sonettobuf::ChargeInfo {
            id: Some(c.charge_id),
            buy_count: Some(c.buy_count),
            first_charge: Some(c.first_charge),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct SandboxSettings {
    pub user_id: i64,
    pub sandbox_enable: bool,
    pub sandbox_balance: i32,
}
