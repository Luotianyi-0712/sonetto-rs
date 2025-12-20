use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct ProcessingStory {
    pub id: i64,
    pub user_id: i64,
    pub story_id: i32,
    pub step_id: i32,
    pub favor: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<ProcessingStory> for sonettobuf::ProcessingStoryInfo {
    fn from(s: ProcessingStory) -> Self {
        sonettobuf::ProcessingStoryInfo {
            story_id: Some(s.story_id),
            step_id: Some(s.step_id),
            favor: Some(s.favor),
        }
    }
}
