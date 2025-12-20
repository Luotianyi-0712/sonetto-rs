use crate::models::game::hero_groups::HeroGroupInfo;
use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct UserDungeon {
    pub id: i64,
    pub user_id: i64,
    pub chapter_id: i32,
    pub episode_id: i32,
    pub star: i32,
    pub challenge_count: i32,
    pub has_record: bool,
    pub left_return_all_num: i32,
    pub today_pass_num: i32,
    pub today_total_num: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<UserDungeon> for sonettobuf::UserDungeon {
    fn from(d: UserDungeon) -> Self {
        sonettobuf::UserDungeon {
            chapter_id: Some(d.chapter_id),
            episode_id: Some(d.episode_id),
            star: Some(d.star),
            challenge_count: Some(d.challenge_count),
            has_record: Some(d.has_record),
            left_return_all_num: Some(d.left_return_all_num),
            today_pass_num: Some(d.today_pass_num),
            today_total_num: Some(d.today_total_num),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DungeonLastHeroGroup {
    pub chapter_id: i32,
    pub hero_group_info: HeroGroupInfo,
}

impl From<DungeonLastHeroGroup> for sonettobuf::DungeonLastHeroGroup {
    fn from(d: DungeonLastHeroGroup) -> Self {
        sonettobuf::DungeonLastHeroGroup {
            chapter_id: Some(d.chapter_id),
            hero_group_snapshot: Some(d.hero_group_info.into()),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct RewardPointInfo {
    pub chapter_id: i32,
    pub reward_point: i32,
    pub has_get_point_reward_ids: Vec<i32>,
}

impl From<RewardPointInfo> for sonettobuf::RewardPointInfo {
    fn from(r: RewardPointInfo) -> Self {
        sonettobuf::RewardPointInfo {
            chapter_id: Some(r.chapter_id),
            reward_point: Some(r.reward_point),
            has_get_point_reward_ids: r.has_get_point_reward_ids,
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct UserChapterTypeNum {
    pub chapter_type: i32,
    pub today_pass_num: i32,
    pub today_total_num: i32,
}

impl From<UserChapterTypeNum> for sonettobuf::UserChapterTypeNum {
    fn from(u: UserChapterTypeNum) -> Self {
        sonettobuf::UserChapterTypeNum {
            chapter_type: Some(u.chapter_type),
            today_pass_num: Some(u.today_pass_num),
            today_total_num: Some(u.today_total_num),
        }
    }
}
