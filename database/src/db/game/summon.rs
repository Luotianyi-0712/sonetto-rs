use crate::models::game::summon::*;
use anyhow::Result;
use sqlx::SqlitePool;

pub async fn get_summon_stats(pool: &SqlitePool, user_id: i64) -> Result<UserSummonStats> {
    let stats =
        sqlx::query_as::<_, UserSummonStats>("SELECT * FROM user_summon_stats WHERE user_id = ?")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

    Ok(stats.unwrap_or(UserSummonStats {
        user_id,
        free_equip_summon: false,
        is_show_new_summon: false,
        new_summon_count: 0,
        total_summon_count: 0,
    }))
}

pub async fn get_summon_pool_infos(pool: &SqlitePool, user_id: i64) -> Result<Vec<SummonPoolInfo>> {
    let pools = sqlx::query_as::<_, UserSummonPool>(
        "SELECT * FROM user_summon_pools WHERE user_id = ? ORDER BY pool_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();
    for pool_data in pools {
        // Get lucky bag info
        let lucky_bag = get_lucky_bag_info(pool, user_id, pool_data.pool_id).await?;

        // Get sp pool info
        let sp_pool = get_sp_pool_info(pool, user_id, pool_data.pool_id).await?;

        result.push(SummonPoolInfo {
            pool: pool_data,
            lucky_bag,
            sp_pool,
        });
    }

    Ok(result)
}

async fn get_lucky_bag_info(
    pool: &SqlitePool,
    user_id: i64,
    pool_id: i32,
) -> Result<Option<LuckyBagInfo>> {
    let bag_data: Option<(i32, i32)> = sqlx::query_as(
        "SELECT count, not_ssr_count FROM user_lucky_bags WHERE user_id = ? AND pool_id = ?",
    )
    .bind(user_id)
    .bind(pool_id)
    .fetch_optional(pool)
    .await?;

    if let Some((count, not_ssr_count)) = bag_data {
        let single_bags: Vec<(i32, bool)> = sqlx::query_as(
            "SELECT bag_id, is_open FROM user_single_bags WHERE user_id = ? AND pool_id = ? ORDER BY bag_id"
        )
        .bind(user_id)
        .bind(pool_id)
        .fetch_all(pool)
        .await?;

        Ok(Some(LuckyBagInfo {
            count,
            single_bag_infos: single_bags
                .into_iter()
                .map(|(bag_id, is_open)| SingleBagInfo { bag_id, is_open })
                .collect(),
            not_ssr_count,
        }))
    } else {
        Ok(None)
    }
}

async fn get_sp_pool_info(
    pool: &SqlitePool,
    user_id: i64,
    pool_id: i32,
) -> Result<Option<SpPoolInfo>> {
    let sp_data: Option<(i32, i32, i32, i64, bool)> = sqlx::query_as(
        "SELECT sp_type, limited_ticket_id, limited_ticket_num, open_time, used_first_ssr_guarantee
         FROM user_sp_pool_info WHERE user_id = ? AND pool_id = ?",
    )
    .bind(user_id)
    .bind(pool_id)
    .fetch_optional(pool)
    .await?;

    if let Some((
        sp_type,
        limited_ticket_id,
        limited_ticket_num,
        open_time,
        used_first_ssr_guarantee,
    )) = sp_data
    {
        let up_hero_ids = sqlx::query_scalar(
            "SELECT hero_id FROM user_sp_pool_up_heroes WHERE user_id = ? AND pool_id = ? ORDER BY hero_id"
        )
        .bind(user_id)
        .bind(pool_id)
        .fetch_all(pool)
        .await?;

        let has_get_reward_progresses = sqlx::query_scalar(
            "SELECT progress_id FROM user_sp_pool_reward_progress WHERE user_id = ? AND pool_id = ? ORDER BY progress_id"
        )
        .bind(user_id)
        .bind(pool_id)
        .fetch_all(pool)
        .await?;

        Ok(Some(SpPoolInfo {
            sp_type,
            up_hero_ids,
            limited_ticket_id,
            limited_ticket_num,
            open_time: open_time as u64,
            used_first_ssr_guarantee,
            has_get_reward_progresses,
        }))
    } else {
        Ok(None)
    }
}
