use crate::models::game::hero_groups::HeroGroupInfo;
use sonettobuf;
use sqlx::FromRow;

#[allow(dead_code)]
#[derive(Debug, Clone, FromRow)]
pub struct HeroGroupSnapshot {
    pub id: i64,
    pub user_id: i64,
    pub snapshot_id: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, FromRow)]
pub struct HeroGroupSnapshotGroup {
    pub id: i64,
    pub snapshot_id: i64,
    pub group_id: i32,
    pub name: String,
    pub cloth_id: i32,
    pub assist_boss_id: i32,
}

#[derive(Debug, Clone)]
pub struct HeroGroupSnapshotInfo {
    pub snapshot_id: i32,
    pub hero_group_snapshots: Vec<HeroGroupInfo>,
    pub sort_sub_ids: Vec<i32>,
}

impl From<HeroGroupSnapshotInfo> for sonettobuf::HeroGroupSnapshotNo {
    fn from(info: HeroGroupSnapshotInfo) -> Self {
        sonettobuf::HeroGroupSnapshotNo {
            snapshot_id: Some(info.snapshot_id),
            hero_group_snapshots: info
                .hero_group_snapshots
                .into_iter()
                .map(Into::into)
                .collect(),
            sort_sub_ids: info.sort_sub_ids,
        }
    }
}
