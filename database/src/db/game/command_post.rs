use crate::models::game::command_post::*;
use anyhow::Result;
use sqlx::SqlitePool;

pub async fn get_command_post_info(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<(
    UserCommandPostInfo,
    Vec<CommandPostEventInfo>,
    Vec<CommandPostTask>,
    Vec<CommandPostTask>,
    Vec<i32>,
)> {
    // Get main info
    let info = sqlx::query_as::<_, UserCommandPostInfo>(
        "SELECT * FROM user_command_post_info WHERE user_id = ?",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .unwrap_or(UserCommandPostInfo {
        user_id,
        paper: 0,
        catch_num: 0,
    });

    // Get events
    let events = get_command_post_events(pool, user_id).await?;

    // Get tasks
    let tasks = sqlx::query_as::<_, CommandPostTask>(
        "SELECT task_id, progress, has_finished, finish_count, task_type, expiry_time
         FROM user_command_post_tasks WHERE user_id = ? ORDER BY task_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    // Get catch tasks
    let catch_tasks = sqlx::query_as::<_, CommandPostTask>(
        "SELECT task_id, progress, has_finished, finish_count, task_type, expiry_time
         FROM user_command_post_catch_tasks WHERE user_id = ? ORDER BY task_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    // Get gain bonuses
    let gain_bonus = sqlx::query_scalar(
        "SELECT bonus_id FROM user_command_post_gain_bonus WHERE user_id = ? ORDER BY bonus_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok((info, events, tasks, catch_tasks, gain_bonus))
}

async fn get_command_post_events(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<CommandPostEventInfo>> {
    let event_data: Vec<(i32, i32, i64, i64, bool)> = sqlx::query_as(
        "SELECT event_id, state, start_time, end_time, is_read
         FROM user_command_post_events WHERE user_id = ? ORDER BY event_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut events = Vec::new();
    for (event_id, state, start_time, end_time, is_read) in event_data {
        let hero_ids = sqlx::query_scalar(
            "SELECT hero_id FROM user_command_post_event_heroes WHERE user_id = ? AND event_id = ? ORDER BY hero_id"
        )
        .bind(user_id)
        .bind(event_id)
        .fetch_all(pool)
        .await?;

        events.push(CommandPostEventInfo {
            event_id,
            state,
            hero_ids,
            start_time: start_time as u64,
            end_time: end_time as u64,
            is_read,
        });
    }

    Ok(events)
}
