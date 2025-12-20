use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Building {
    pub uid: i64,
    pub user_id: i64,
    pub define_id: i32,
    pub in_use: bool,
    pub x: i32,
    pub y: i32,
    pub rotate: i32,
    pub level: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Building> for sonettobuf::BuildingInfo {
    fn from(b: Building) -> Self {
        sonettobuf::BuildingInfo {
            uid: Some(b.uid),
            define_id: Some(b.define_id),
            r#use: Some(b.in_use),
            x: Some(b.x),
            y: Some(b.y),
            rotate: Some(b.rotate),
            level: Some(b.level),
        }
    }
}
