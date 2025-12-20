use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Item {
    pub user_id: i64,
    pub item_id: i64,
    pub quantity: i32,
    pub last_use_time: Option<i64>,
    pub last_update_time: Option<i64>,
    pub total_gain_count: i64,
}

impl From<Item> for sonettobuf::Item {
    fn from(item: Item) -> Self {
        sonettobuf::Item {
            item_id: Some(item.item_id as u32),
            quantity: Some(item.quantity),
            last_use_time: item.last_use_time.map(|t| t as u64),
            last_update_time: item.last_update_time.map(|t| t as u64),
            total_gain_count: Some(item.total_gain_count),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct PowerItem {
    pub uid: i64,
    pub user_id: i64,
    pub item_id: i64,
    pub quantity: i32,
    pub expire_time: i32,
}

impl From<PowerItem> for sonettobuf::PowerItem {
    fn from(item: PowerItem) -> Self {
        sonettobuf::PowerItem {
            uid: Some(item.uid),
            item_id: Some(item.item_id),
            quantity: Some(item.quantity),
            expire_time: Some(item.expire_time),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct InsightItem {
    pub uid: i64,
    pub user_id: i64,
    pub item_id: i32,
    pub quantity: i32,
    pub expire_time: i32,
}

impl From<InsightItem> for sonettobuf::InsightItem {
    fn from(item: InsightItem) -> Self {
        sonettobuf::InsightItem {
            uid: Some(item.uid),
            item_id: Some(item.item_id),
            quantity: Some(item.quantity),
            expire_time: Some(item.expire_time),
        }
    }
}
