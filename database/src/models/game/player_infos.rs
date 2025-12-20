use serde::{Deserialize, Serialize};
use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PlayerInfo {
    pub player_id: i64,
    pub signature: String,
    pub birthday: String,
    pub portrait: i32,
    pub show_achievement: String,
    pub bg: i32,
    pub total_login_days: i32,
    pub last_episode_id: i32,
    pub last_logout_time: Option<i64>,
    pub hero_rare_nn_count: i32,
    pub hero_rare_n_count: i32,
    pub hero_rare_r_count: i32,
    pub hero_rare_sr_count: i32,
    pub hero_rare_ssr_count: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ShowHero {
    pub id: i64,
    pub player_id: i64,
    pub hero_id: i32,
    pub level: i32,
    pub rank: i32,
    pub ex_skill_level: i32,
    pub skin: i32,
    pub display_order: i32,
}

impl From<ShowHero> for sonettobuf::HeroSimpleInfo {
    fn from(h: ShowHero) -> Self {
        sonettobuf::HeroSimpleInfo {
            hero_id: h.hero_id,
            level: Some(h.level),
            rank: Some(h.rank),
            ex_skill_level: Some(h.ex_skill_level),
            skin: Some(h.skin),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserBasicInfo {
    pub username: String,
    pub level: i32,
    pub exp: i32,
    pub created_at: Option<i64>,
    pub last_login_at: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct PlayerInfoData {
    pub player_id: i64,
    pub user_info: UserBasicInfo,
    pub player_info: PlayerInfo,
    pub show_heroes: Vec<ShowHero>,
}

impl From<PlayerInfoData> for sonettobuf::PlayerInfo {
    fn from(data: PlayerInfoData) -> Self {
        sonettobuf::PlayerInfo {
            user_id: Some(data.player_id as u64),
            name: Some(data.user_info.username),
            portrait: Some(data.player_info.portrait),
            level: Some(data.user_info.level),
            exp: Some(data.user_info.exp),
            signature: Some(data.player_info.signature),
            birthday: Some(data.player_info.birthday),
            show_heros: data.show_heroes.into_iter().map(Into::into).collect(),
            register_time: data.user_info.created_at,
            hero_rare_nn_count: Some(data.player_info.hero_rare_nn_count),
            hero_rare_n_count: Some(data.player_info.hero_rare_n_count),
            hero_rare_r_count: Some(data.player_info.hero_rare_r_count),
            hero_rare_sr_count: Some(data.player_info.hero_rare_sr_count),
            hero_rare_ssr_count: Some(data.player_info.hero_rare_ssr_count),
            last_episode_id: Some(data.player_info.last_episode_id),
            last_login_time: data.user_info.last_login_at,
            last_logout_time: data.player_info.last_logout_time,
            character_age: vec![],
            show_achievement: Some(data.player_info.show_achievement),
            bg: Some(data.player_info.bg),
            total_login_days: Some(data.player_info.total_login_days),
        }
    }
}
