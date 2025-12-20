use crate::models::game::player_card::PlayerCardInfo;
use anyhow::Result;
use sqlx::SqlitePool;

pub async fn get_player_card_info(pool: &SqlitePool, user_id: i64) -> Result<PlayerCardInfo> {
    let info = sqlx::query_as::<_, PlayerCardInfo>(
        "SELECT * FROM user_player_card_info WHERE user_id = ?",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .unwrap_or(PlayerCardInfo {
        user_id,
        show_settings: String::new(),
        progress_setting: String::new(),
        base_setting: String::new(),
        hero_cover: String::new(),
        theme_id: 0,
        show_achievement: String::new(),
        critter: String::new(),
        room_collection: String::new(),
        weekwalk_deep_layer_id: 0,
        explore_collection: String::new(),
        rouge_difficulty: 0,
        act128_sss_count: 0,
        achievement_count: 0,
        assist_times: 0,
        hero_cover_times: 0,
        max_faith_hero_count: 0,
        total_cost_power: 0,
        skin_count: 0,
        tower_layer: 0,
        tower_boss_pass_count: 0,
        hero_max_level_count: 0,
        weekwalk_ver2_platinum_cup: 0,
        hero_count: 0,
        tower_layer_metre: 0,
    });

    Ok(info)
}

pub async fn update_player_card_info(pool: &SqlitePool, info: &PlayerCardInfo) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO user_player_card_info (
            user_id, show_settings, progress_setting, base_setting, hero_cover, theme_id,
            show_achievement, critter, room_collection, weekwalk_deep_layer_id, explore_collection,
            rouge_difficulty, act128_sss_count, achievement_count, assist_times, hero_cover_times,
            max_faith_hero_count, total_cost_power, skin_count, tower_layer, tower_boss_pass_count,
            hero_max_level_count, weekwalk_ver2_platinum_cup, hero_count, tower_layer_metre
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(user_id) DO UPDATE SET
            show_settings = excluded.show_settings,
            progress_setting = excluded.progress_setting,
            base_setting = excluded.base_setting,
            hero_cover = excluded.hero_cover,
            theme_id = excluded.theme_id,
            show_achievement = excluded.show_achievement,
            critter = excluded.critter,
            room_collection = excluded.room_collection,
            weekwalk_deep_layer_id = excluded.weekwalk_deep_layer_id,
            explore_collection = excluded.explore_collection,
            rouge_difficulty = excluded.rouge_difficulty,
            act128_sss_count = excluded.act128_sss_count,
            achievement_count = excluded.achievement_count,
            assist_times = excluded.assist_times,
            hero_cover_times = excluded.hero_cover_times,
            max_faith_hero_count = excluded.max_faith_hero_count,
            total_cost_power = excluded.total_cost_power,
            skin_count = excluded.skin_count,
            tower_layer = excluded.tower_layer,
            tower_boss_pass_count = excluded.tower_boss_pass_count,
            hero_max_level_count = excluded.hero_max_level_count,
            weekwalk_ver2_platinum_cup = excluded.weekwalk_ver2_platinum_cup,
            hero_count = excluded.hero_count,
            tower_layer_metre = excluded.tower_layer_metre
        "#,
    )
    .bind(info.user_id)
    .bind(&info.show_settings)
    .bind(&info.progress_setting)
    .bind(&info.base_setting)
    .bind(&info.hero_cover)
    .bind(info.theme_id)
    .bind(&info.show_achievement)
    .bind(&info.critter)
    .bind(&info.room_collection)
    .bind(info.weekwalk_deep_layer_id)
    .bind(&info.explore_collection)
    .bind(info.rouge_difficulty)
    .bind(info.act128_sss_count)
    .bind(info.achievement_count)
    .bind(info.assist_times)
    .bind(info.hero_cover_times)
    .bind(info.max_faith_hero_count)
    .bind(info.total_cost_power)
    .bind(info.skin_count)
    .bind(info.tower_layer)
    .bind(info.tower_boss_pass_count)
    .bind(info.hero_max_level_count)
    .bind(info.weekwalk_ver2_platinum_cup)
    .bind(info.hero_count)
    .bind(info.tower_layer_metre)
    .execute(pool)
    .await?;

    Ok(())
}
