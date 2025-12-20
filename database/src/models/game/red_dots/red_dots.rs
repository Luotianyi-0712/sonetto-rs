use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct RedDotRecord {
    pub id: i64,
    pub player_id: i64,
    pub define_id: i32,
    pub info_id: i32,
    pub value: i32,
    pub time: i64,
    pub ext: String,
    pub replace_all: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<RedDotRecord> for sonettobuf::RedDotInfo {
    fn from(dot: RedDotRecord) -> Self {
        sonettobuf::RedDotInfo {
            id: dot.info_id as i64,
            value: dot.value,
            time: Some(dot.time as i32),
            ext: Some(dot.ext),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RedDotGroup {
    pub define_id: i32,
    pub dots: Vec<RedDotRecord>,
    pub replace_all: bool,
}

impl From<RedDotGroup> for sonettobuf::RedDotGroup {
    fn from(group: RedDotGroup) -> Self {
        sonettobuf::RedDotGroup {
            define_id: group.define_id,
            infos: group.dots.into_iter().map(Into::into).collect(),
            replace_all: Some(group.replace_all),
        }
    }
}
