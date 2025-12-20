use crate::models::game::guides::GuideProgress;
use sqlx::SqlitePool;

pub async fn get_all_guide_progress(
    pool: &SqlitePool,
    user_id: i64,
) -> sqlx::Result<Vec<GuideProgress>> {
    sqlx::query_as::<_, GuideProgress>(
        "SELECT user_id, guide_id, step_id FROM guide_progress WHERE user_id = ? ORDER BY guide_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_guide_progress(
    pool: &SqlitePool,
    user_id: i64,
    guide_id: i32,
) -> sqlx::Result<Option<GuideProgress>> {
    sqlx::query_as::<_, GuideProgress>(
        "SELECT user_id, guide_id, step_id FROM guide_progress WHERE user_id = ? AND guide_id = ?",
    )
    .bind(user_id)
    .bind(guide_id)
    .fetch_optional(pool)
    .await
}

pub async fn update_guide_progress(
    pool: &SqlitePool,
    user_id: i64,
    guide_id: i32,
    step_id: i32,
) -> sqlx::Result<()> {
    sqlx::query(
        "INSERT INTO guide_progress (user_id, guide_id, step_id)
         VALUES (?, ?, ?)
         ON CONFLICT(user_id, guide_id) DO UPDATE SET step_id = excluded.step_id",
    )
    .bind(user_id)
    .bind(guide_id)
    .bind(step_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn complete_guide(pool: &SqlitePool, user_id: i64, guide_id: i32) -> sqlx::Result<()> {
    update_guide_progress(pool, user_id, guide_id, -1).await
}
