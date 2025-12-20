use anyhow::Result;
use sqlx::SqlitePool;

pub use crate::models::game::heros::*;

/// Get all heroes for a user
pub async fn get_user_heroes(pool: &SqlitePool, user_id: i64) -> Result<Vec<HeroData>> {
    let heroes = sqlx::query_as::<_, Hero>("SELECT * FROM heroes WHERE user_id = ?1 ORDER BY uid")
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    let mut result = Vec::new();

    for hero_record in heroes {
        let hero_uid = hero_record.uid;

        // Get passive skill levels
        let passive_skill_levels: Vec<i32> = sqlx::query_scalar(
            "SELECT level FROM hero_passive_skill_levels WHERE hero_uid = ?1 ORDER BY skill_index",
        )
        .bind(hero_uid)
        .fetch_all(pool)
        .await?;

        // Get voices
        let voices: Vec<i32> =
            sqlx::query_scalar("SELECT voice_id FROM hero_voices WHERE hero_uid = ?1")
                .bind(hero_uid)
                .fetch_all(pool)
                .await?;

        // Get voices heard
        let voices_heard: Vec<i32> =
            sqlx::query_scalar("SELECT voice_id FROM hero_voices_heard WHERE hero_uid = ?1")
                .bind(hero_uid)
                .fetch_all(pool)
                .await?;

        // Get skins
        let skin_list = sqlx::query_as::<_, HeroSkin>(
            "SELECT hero_uid, skin, expire_sec FROM hero_skins WHERE hero_uid = ?1",
        )
        .bind(hero_uid)
        .fetch_all(pool)
        .await?;

        // Get sp attributes
        let sp_attr =
            sqlx::query_as::<_, HeroSpAttribute>("SELECT * FROM hero_sp_attrs WHERE hero_uid = ?1")
                .bind(hero_uid)
                .fetch_optional(pool)
                .await?;

        // Get equip attributes
        let equip_attrs = sqlx::query_as::<_, HeroEquipAttribute>(
            "SELECT * FROM hero_equip_attributes WHERE hero_uid = ?1",
        )
        .bind(hero_uid)
        .fetch_all(pool)
        .await?;

        // Get item unlocks
        let item_unlocks: Vec<i32> =
            sqlx::query_scalar("SELECT item_id FROM hero_item_unlocks WHERE hero_uid = ?1")
                .bind(hero_uid)
                .fetch_all(pool)
                .await?;

        // Get talent cubes
        let talent_cubes = sqlx::query_as::<_, HeroTalentCube>(
            "SELECT hero_uid, cube_id, direction, pos_x, pos_y FROM hero_talent_cubes WHERE hero_uid = ?1"
        )
        .bind(hero_uid)
        .fetch_all(pool)
        .await?;

        // Get talent templates with their cubes
        let templates = sqlx::query_as::<_, HeroTalentTemplate>(
            "SELECT id, hero_uid, template_id, name, style FROM hero_talent_templates WHERE hero_uid = ?1"
        )
        .bind(hero_uid)
        .fetch_all(pool)
        .await?;

        let mut talent_templates = Vec::new();
        for template in templates {
            let template_cubes = sqlx::query_as::<_, HeroTalentCube>(
                "SELECT 0 as hero_uid, cube_id, direction, pos_x, pos_y
                 FROM hero_talent_template_cubes WHERE template_row_id = ?1",
            )
            .bind(template.id)
            .fetch_all(pool)
            .await?;

            talent_templates.push((template, template_cubes));
        }

        // Get destiny stone unlocks
        let destiny_stone_unlocks: Vec<i32> = sqlx::query_scalar(
            "SELECT stone_id FROM hero_destiny_stone_unlocks WHERE hero_uid = ?1",
        )
        .bind(hero_uid)
        .fetch_all(pool)
        .await?;

        result.push(HeroData {
            record: hero_record,
            passive_skill_levels,
            voices,
            voices_heard,
            skin_list,
            sp_attr,
            equip_attrs,
            item_unlocks,
            talent_cubes,
            talent_templates,
            destiny_stone_unlocks,
        });
    }

    Ok(result)
}

/// Get a single hero by hero_id (not uid)
pub async fn get_hero_by_hero_id(
    pool: &SqlitePool,
    user_id: i64,
    hero_id: i32,
) -> Result<HeroData> {
    let hero_record =
        sqlx::query_as::<_, Hero>("SELECT * FROM heroes WHERE user_id = ? AND hero_id = ?")
            .bind(user_id)
            .bind(hero_id)
            .fetch_one(pool)
            .await?;

    let hero_uid = hero_record.uid;

    // Get passive skill levels
    let passive_skill_levels: Vec<i32> = sqlx::query_scalar(
        "SELECT level FROM hero_passive_skill_levels WHERE hero_uid = ? ORDER BY skill_index",
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    // Get voices
    let voices: Vec<i32> =
        sqlx::query_scalar("SELECT voice_id FROM hero_voices WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_all(pool)
            .await?;

    // Get voices heard
    let voices_heard: Vec<i32> =
        sqlx::query_scalar("SELECT voice_id FROM hero_voices_heard WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_all(pool)
            .await?;

    // Get skins
    let skin_list = sqlx::query_as::<_, HeroSkin>(
        "SELECT hero_uid, skin, expire_sec FROM hero_skins WHERE hero_uid = ?",
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    // Get sp attributes
    let sp_attr =
        sqlx::query_as::<_, HeroSpAttribute>("SELECT * FROM hero_sp_attrs WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_optional(pool)
            .await?;

    // Get equip attributes
    let equip_attrs = sqlx::query_as::<_, HeroEquipAttribute>(
        "SELECT * FROM hero_equip_attributes WHERE hero_uid = ?",
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    // Get item unlocks
    let item_unlocks: Vec<i32> =
        sqlx::query_scalar("SELECT item_id FROM hero_item_unlocks WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_all(pool)
            .await?;

    // Get talent cubes
    let talent_cubes = sqlx::query_as::<_, HeroTalentCube>(
        "SELECT hero_uid, cube_id, direction, pos_x, pos_y FROM hero_talent_cubes WHERE hero_uid = ?"
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    // Get talent templates with their cubes
    let templates = sqlx::query_as::<_, HeroTalentTemplate>(
        "SELECT id, hero_uid, template_id, name, style FROM hero_talent_templates WHERE hero_uid = ?"
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    let mut talent_templates = Vec::new();
    for template in templates {
        let template_cubes = sqlx::query_as::<_, HeroTalentCube>(
            "SELECT 0 as hero_uid, cube_id, direction, pos_x, pos_y
             FROM hero_talent_template_cubes WHERE template_row_id = ?",
        )
        .bind(template.id)
        .fetch_all(pool)
        .await?;

        talent_templates.push((template, template_cubes));
    }

    // Get destiny stone unlocks
    let destiny_stone_unlocks: Vec<i32> =
        sqlx::query_scalar("SELECT stone_id FROM hero_destiny_stone_unlocks WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_all(pool)
            .await?;

    Ok(HeroData {
        record: hero_record,
        passive_skill_levels,
        voices,
        voices_heard,
        skin_list,
        sp_attr,
        equip_attrs,
        item_unlocks,
        talent_cubes,
        talent_templates,
        destiny_stone_unlocks,
    })
}

/// Get a single hero by hero uid
pub async fn get_hero_by_hero_uid(
    pool: &SqlitePool,
    user_id: i64,
    hero_uid: i32,
) -> Result<HeroData> {
    let hero_record =
        sqlx::query_as::<_, Hero>("SELECT * FROM heroes WHERE user_id = ? AND uid = ?")
            .bind(user_id)
            .bind(hero_uid)
            .fetch_one(pool)
            .await?;

    let hero_uid = hero_record.uid;

    // Get passive skill levels
    let passive_skill_levels: Vec<i32> = sqlx::query_scalar(
        "SELECT level FROM hero_passive_skill_levels WHERE hero_uid = ? ORDER BY skill_index",
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    // Get voices
    let voices: Vec<i32> =
        sqlx::query_scalar("SELECT voice_id FROM hero_voices WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_all(pool)
            .await?;

    // Get voices heard
    let voices_heard: Vec<i32> =
        sqlx::query_scalar("SELECT voice_id FROM hero_voices_heard WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_all(pool)
            .await?;

    // Get skins
    let skin_list = sqlx::query_as::<_, HeroSkin>(
        "SELECT hero_uid, skin, expire_sec FROM hero_skins WHERE hero_uid = ?",
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    // Get sp attributes
    let sp_attr =
        sqlx::query_as::<_, HeroSpAttribute>("SELECT * FROM hero_sp_attrs WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_optional(pool)
            .await?;

    // Get equip attributes
    let equip_attrs = sqlx::query_as::<_, HeroEquipAttribute>(
        "SELECT * FROM hero_equip_attributes WHERE hero_uid = ?",
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    // Get item unlocks
    let item_unlocks: Vec<i32> =
        sqlx::query_scalar("SELECT item_id FROM hero_item_unlocks WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_all(pool)
            .await?;

    // Get talent cubes
    let talent_cubes = sqlx::query_as::<_, HeroTalentCube>(
        "SELECT hero_uid, cube_id, direction, pos_x, pos_y FROM hero_talent_cubes WHERE hero_uid = ?"
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    // Get talent templates with their cubes
    let templates = sqlx::query_as::<_, HeroTalentTemplate>(
        "SELECT id, hero_uid, template_id, name, style FROM hero_talent_templates WHERE hero_uid = ?"
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    let mut talent_templates = Vec::new();
    for template in templates {
        let template_cubes = sqlx::query_as::<_, HeroTalentCube>(
            "SELECT 0 as hero_uid, cube_id, direction, pos_x, pos_y
             FROM hero_talent_template_cubes WHERE template_row_id = ?",
        )
        .bind(template.id)
        .fetch_all(pool)
        .await?;

        talent_templates.push((template, template_cubes));
    }

    // Get destiny stone unlocks
    let destiny_stone_unlocks: Vec<i32> =
        sqlx::query_scalar("SELECT stone_id FROM hero_destiny_stone_unlocks WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_all(pool)
            .await?;

    Ok(HeroData {
        record: hero_record,
        passive_skill_levels,
        voices,
        voices_heard,
        skin_list,
        sp_attr,
        equip_attrs,
        item_unlocks,
        talent_cubes,
        talent_templates,
        destiny_stone_unlocks,
    })
}

/// Get touch count for user
pub async fn get_touch_count(pool: &SqlitePool, user_id: i64) -> Result<Option<i32>> {
    let count: Option<i32> =
        sqlx::query_scalar("SELECT touch_count_left FROM hero_touch_count WHERE user_id = ?1")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

    Ok(count)
}

/// Decrement touch count (returns new count, or None if no touches left)
pub async fn use_touch(pool: &SqlitePool, user_id: i64) -> Result<Option<i32>> {
    // Get current count
    let current = get_touch_count(pool, user_id).await?;
    let current = current.unwrap_or(5);

    if current <= 0 {
        return Ok(None); // No touches left
    }

    // Decrement
    let new_count = current - 1;
    sqlx::query("UPDATE hero_touch_count SET touch_count_left = ? WHERE user_id = ?")
        .bind(new_count)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(Some(new_count))
}

/// Reset daily touch count
pub async fn reset_touch_count(pool: &SqlitePool, user_id: i64) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO hero_touch_count (user_id, touch_count_left)
        VALUES (?, 5)
        ON CONFLICT(user_id) DO UPDATE SET touch_count_left = 5
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Get all hero skins for user
pub async fn get_all_hero_skins(pool: &SqlitePool, user_id: i64) -> Result<Vec<i32>> {
    let skins: Vec<i32> =
        sqlx::query_scalar("SELECT skin_id FROM hero_all_skins WHERE user_id = ?1")
            .bind(user_id)
            .fetch_all(pool)
            .await?;

    Ok(skins)
}

/// Get birthday info for user
pub async fn get_birthday_info(pool: &SqlitePool, user_id: i64) -> Result<Vec<(i32, i32)>> {
    let info: Vec<(i32, i32)> =
        sqlx::query_as("SELECT hero_id, birthday_count FROM hero_birthday_info WHERE user_id = ?1")
            .bind(user_id)
            .fetch_all(pool)
            .await?;

    Ok(info)
}
