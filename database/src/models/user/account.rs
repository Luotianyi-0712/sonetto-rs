use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    #[sqlx(rename = "Uid")]
    pub uid: i64,

    #[sqlx(rename = "UserName")]
    pub user_name: String,

    #[sqlx(rename = "Password")]
    pub password: String,
}
