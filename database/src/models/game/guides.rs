use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct GuideProgress {
    pub user_id: i64,
    pub guide_id: i32,
    pub step_id: i32,
}

impl From<GuideProgress> for sonettobuf::GuideInfo {
    fn from(g: GuideProgress) -> Self {
        sonettobuf::GuideInfo {
            guide_id: g.guide_id,
            step_id: g.step_id,
        }
    }
}
