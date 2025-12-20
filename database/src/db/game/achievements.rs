use crate::models::game::achievements::Achievement;
use anyhow::Result;
use sqlx::SqlitePool;

pub async fn get_achievements(pool: &SqlitePool, user_id: i64) -> Result<Vec<Achievement>> {
    let achievements = sqlx::query_as::<_, Achievement>(
        "SELECT * FROM user_achievements WHERE user_id = ? ORDER BY achievement_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(achievements)
}

pub async fn update_achievement_progress(
    pool: &SqlitePool,
    user_id: i64,
    achievement_id: i32,
    progress: i32,
) -> Result<()> {
    let now = chrono::Utc::now().timestamp();

    sqlx::query(
        r#"
        INSERT INTO user_achievements (
            user_id, achievement_id, progress, has_finish, is_new, finish_time, created_at, updated_at
        ) VALUES (?, ?, ?, 0, 1, 0, ?, ?)
        ON CONFLICT(user_id, achievement_id) DO UPDATE SET
            progress = excluded.progress,
            is_new = 1,
            updated_at = excluded.updated_at
        "#
    )
    .bind(user_id)
    .bind(achievement_id)
    .bind(progress)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn finish_achievement(
    pool: &SqlitePool,
    user_id: i64,
    achievement_id: i32,
) -> Result<()> {
    let now = chrono::Utc::now().timestamp();
    let finish_time = now as i32;

    sqlx::query(
        r#"
        UPDATE user_achievements
        SET has_finish = 1, finish_time = ?, is_new = 1, updated_at = ?
        WHERE user_id = ? AND achievement_id = ?
        "#,
    )
    .bind(finish_time)
    .bind(now)
    .bind(user_id)
    .bind(achievement_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn clear_new_flag(pool: &SqlitePool, user_id: i64, achievement_id: i32) -> Result<()> {
    let now = chrono::Utc::now().timestamp();

    sqlx::query(
        "UPDATE user_achievements SET is_new = 0, updated_at = ? WHERE user_id = ? AND achievement_id = ?"
    )
    .bind(now)
    .bind(user_id)
    .bind(achievement_id)
    .execute(pool)
    .await?;

    Ok(())
}
