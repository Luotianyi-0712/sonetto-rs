use anyhow::Result;
use sqlx::{SqlitePool, prelude::FromRow};

pub use crate::models::game::player_infos::{PlayerInfo, PlayerInfoData, ShowHero, UserBasicInfo};

pub async fn get_player_info_data(
    pool: &SqlitePool,
    player_id: i64,
) -> anyhow::Result<Option<PlayerInfoData>> {
    // Get user basic info
    let user_info = sqlx::query_as::<_, (String, i32, i32, Option<i64>, Option<i64>)>(
        "SELECT username, level, exp, created_at, last_login_at FROM users WHERE id = ?",
    )
    .bind(player_id)
    .fetch_optional(pool)
    .await?;

    let Some((username, level, exp, created_at, last_login_at)) = user_info else {
        return Ok(None);
    };

    // Get player info
    let Some(player_info) = get_player_info(pool, player_id).await? else {
        return Ok(None);
    };

    // Get show heroes
    let show_heroes = get_show_heroes(pool, player_id).await?;

    Ok(Some(PlayerInfoData {
        player_id,
        user_info: UserBasicInfo {
            username,
            level,
            exp,
            created_at,
            last_login_at,
        },
        player_info,
        show_heroes,
    }))
}

/// Get player info by player_id
pub async fn get_player_info(pool: &SqlitePool, player_id: i64) -> Result<Option<PlayerInfo>> {
    let record = sqlx::query_as::<_, PlayerInfo>("SELECT * FROM player_info WHERE player_id = ?1")
        .bind(player_id)
        .fetch_optional(pool)
        .await?;

    Ok(record)
}

/// Get player's show heroes
pub async fn get_show_heroes(pool: &SqlitePool, player_id: i64) -> Result<Vec<ShowHero>> {
    let heroes = sqlx::query_as::<_, ShowHero>(
        "SELECT * FROM player_show_heroes WHERE player_id = ?1 ORDER BY display_order",
    )
    .bind(player_id)
    .fetch_all(pool)
    .await?;

    Ok(heroes)
}

pub async fn get_user_basic_info(
    pool: &SqlitePool,
    user_id: i64,
) -> sqlx::Result<(String, i32, i32)> {
    sqlx::query_as::<_, (String, i32, i32)>("SELECT username, level, exp FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await
}

/// Create default player info
pub async fn create_player_info(pool: &SqlitePool, player_id: i64, now: i64) -> Result<()> {
    sqlx::query(
        "INSERT INTO player_info (
            player_id, signature, birthday, portrait, show_achievement, bg,
            total_login_days, last_episode_id, last_logout_time,
            hero_rare_nn_count, hero_rare_n_count, hero_rare_r_count,
            hero_rare_sr_count, hero_rare_ssr_count,
            created_at, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
    )
    .bind(player_id)
    .bind("")
    .bind("")
    .bind(171603)
    .bind("")
    .bind(0)
    .bind(0)
    .bind(0)
    .bind(None::<i64>)
    .bind(0)
    .bind(0)
    .bind(0)
    .bind(0)
    .bind(0)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

/// Update player info
pub async fn update_player_info(pool: &SqlitePool, info: &PlayerInfo) -> Result<()> {
    sqlx::query(
        "UPDATE player_info SET
            signature = ?2,
            birthday = ?3,
            portrait = ?4,
            show_achievement = ?5,
            bg = ?6,
            total_login_days = ?7,
            last_episode_id = ?8,
            last_logout_time = ?9,
            hero_rare_nn_count = ?10,
            hero_rare_n_count = ?11,
            hero_rare_r_count = ?12,
            hero_rare_sr_count = ?13,
            hero_rare_ssr_count = ?14,
            updated_at = ?15
         WHERE player_id = ?1",
    )
    .bind(info.player_id)
    .bind(&info.signature)
    .bind(&info.birthday)
    .bind(info.portrait)
    .bind(&info.show_achievement)
    .bind(info.bg)
    .bind(info.total_login_days)
    .bind(info.last_episode_id)
    .bind(info.last_logout_time)
    .bind(info.hero_rare_nn_count)
    .bind(info.hero_rare_n_count)
    .bind(info.hero_rare_r_count)
    .bind(info.hero_rare_sr_count)
    .bind(info.hero_rare_ssr_count)
    .bind(info.updated_at)
    .execute(pool)
    .await?;

    Ok(())
}

/// Set show heroes
pub async fn set_show_hero(pool: &SqlitePool, player_id: i64, hero_uids: &[i64]) -> Result<()> {
    let mut tx = pool.begin().await?;

    for (slot_idx, uid) in hero_uids.iter().enumerate() {
        let display_order = slot_idx as i32;

        if *uid == 0 {
            // Explicitly clear this slot
            sqlx::query(
                r#"
                DELETE FROM player_show_heroes
                WHERE player_id = ? AND display_order = ?
                "#,
            )
            .bind(player_id)
            .bind(display_order)
            .execute(&mut *tx)
            .await?;

            continue;
        }

        #[derive(FromRow)]
        struct HeroRow {
            hero_id: i32,
            level: i32,
            rank: i32,
            ex_skill_level: i32,
            skin: i32,
        }

        // Resolve hero UID → hero data
        let hero = sqlx::query_as::<_, HeroRow>(
            "
            SELECT
                hero_id,
                level,
                rank,
                ex_skill_level,
                skin
            FROM heroes
            WHERE uid = ? AND user_id = ?
            ",
        )
        .bind(uid)
        .bind(player_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Invalid hero uid {} for user {}", uid, player_id))?;

        // Set / replace this slot
        sqlx::query(
            r#"
            INSERT INTO player_show_heroes (
                player_id,
                hero_id,
                level,
                rank,
                ex_skill_level,
                skin,
                display_order
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(player_id, display_order)
            DO UPDATE SET
                hero_id = excluded.hero_id,
                level = excluded.level,
                rank = excluded.rank,
                ex_skill_level = excluded.ex_skill_level,
                skin = excluded.skin
            "#,
        )
        .bind(player_id)
        .bind(hero.hero_id)
        .bind(hero.level)
        .bind(hero.rank)
        .bind(hero.ex_skill_level)
        .bind(hero.skin)
        .bind(display_order)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}
