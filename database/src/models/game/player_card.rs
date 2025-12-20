use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct PlayerCardInfo {
    pub user_id: i64,
    pub show_settings: String,
    pub progress_setting: String,
    pub base_setting: String,
    pub hero_cover: String,
    pub theme_id: i32,
    pub show_achievement: String,
    pub critter: String,
    pub room_collection: String,
    pub weekwalk_deep_layer_id: i32,
    pub explore_collection: String,
    pub rouge_difficulty: i32,
    pub act128_sss_count: i32,
    pub achievement_count: i32,
    pub assist_times: i32,
    pub hero_cover_times: i32,
    pub max_faith_hero_count: i32,
    pub total_cost_power: i32,
    pub skin_count: i32,
    pub tower_layer: i32,
    pub tower_boss_pass_count: i32,
    pub hero_max_level_count: i32,
    pub weekwalk_ver2_platinum_cup: i32,
    pub hero_count: i32,
    pub tower_layer_metre: i32,
}

impl From<PlayerCardInfo> for sonettobuf::PlayerCardInfo {
    fn from(p: PlayerCardInfo) -> Self {
        // Parse show_settings JSON array string into Vec<String>
        let show_settings = if p.show_settings.is_empty() {
            vec![]
        } else {
            serde_json::from_str::<Vec<String>>(&p.show_settings).unwrap_or_default()
        };

        sonettobuf::PlayerCardInfo {
            show_settings,
            progress_setting: Some(p.progress_setting),
            base_setting: Some(p.base_setting),
            hero_cover: Some(p.hero_cover),
            theme_id: Some(p.theme_id),
            show_achievement: Some(p.show_achievement),
            critter: Some(p.critter),
            room_collection: Some(p.room_collection),
            weekwalk_deep_layer_id: Some(p.weekwalk_deep_layer_id),
            explore_collection: Some(p.explore_collection),
            rouge_difficulty: Some(p.rouge_difficulty),
            act128_sss_count: Some(p.act128_sss_count),
            achievement_count: Some(p.achievement_count),
            assist_times: Some(p.assist_times),
            hero_cover_times: Some(p.hero_cover_times),
            max_faith_hero_count: Some(p.max_faith_hero_count),
            total_cost_power: Some(p.total_cost_power),
            skin_count: Some(p.skin_count),
            tower_layer: Some(p.tower_layer),
            tower_boss_pass_count: Some(p.tower_boss_pass_count),
            hero_max_level_count: Some(p.hero_max_level_count),
            weekwalk_ver2_platinum_cup: Some(p.weekwalk_ver2_platinum_cup),
            hero_count: Some(p.hero_count),
            tower_layer_metre: Some(p.tower_layer_metre),
        }
    }
}
