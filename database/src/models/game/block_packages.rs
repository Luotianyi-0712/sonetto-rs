use sqlx::FromRow;
use sonettobuf;

#[derive(Debug, Clone, FromRow)]
pub struct SpecialBlock {
    pub user_id: i64,
    pub block_id: i32,
    pub create_time: i32,
}

impl From<SpecialBlock> for sonettobuf::SpecialBlockInfo {
    fn from(b: SpecialBlock) -> Self {
        sonettobuf::SpecialBlockInfo {
            block_id: Some(b.block_id),
            create_time: Some(b.create_time),
        }
    }
}
