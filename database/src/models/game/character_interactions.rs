use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct CharacterInteraction {
    pub id: i64,
    pub user_id: i64,
    pub interaction_id: i32,
    pub is_finished: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone)]
pub struct CharacterInteractionInfo {
    pub interaction_id: i32,
    pub is_finished: bool,
    pub select_ids: Vec<i32>,
}

impl From<CharacterInteractionInfo> for sonettobuf::CharacterInteractionInfo {
    fn from(info: CharacterInteractionInfo) -> Self {
        sonettobuf::CharacterInteractionInfo {
            id: Some(info.interaction_id),
            finish: Some(info.is_finished),
            select_ids: info.select_ids,
        }
    }
}
