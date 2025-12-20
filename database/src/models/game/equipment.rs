use serde::{Deserialize, Serialize};
use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Equipment {
    pub uid: i64,
    pub user_id: i64,
    pub equip_id: i32,
    pub level: i32,
    pub exp: i32,
    pub break_lv: i32,
    pub count: i32,
    pub is_lock: bool,
    pub refine_lv: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Equipment> for sonettobuf::Equip {
    fn from(e: Equipment) -> Self {
        sonettobuf::Equip {
            equip_id: Some(e.equip_id),
            uid: Some(e.uid),
            level: Some(e.level),
            exp: Some(e.exp),
            break_lv: Some(e.break_lv),
            count: Some(e.count),
            is_lock: Some(e.is_lock),
            refine_lv: Some(e.refine_lv),
        }
    }
}
