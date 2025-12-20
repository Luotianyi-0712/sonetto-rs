use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Currency {
    pub user_id: i64,
    pub currency_id: i32,
    pub quantity: i32,
    pub last_recover_time: Option<i64>,
    pub expired_time: Option<i64>,
}

impl From<Currency> for sonettobuf::Currency {
    fn from(c: Currency) -> Self {
        sonettobuf::Currency {
            currency_id: Some(c.currency_id as u32),
            quantity: Some(c.quantity),
            last_recover_time: c.last_recover_time.map(|t| t as u64),
            expired_time: c.expired_time.map(|t| t as u64),
        }
    }
}
