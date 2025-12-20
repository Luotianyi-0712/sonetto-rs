use anyhow::Result;
use sqlx::SqlitePool;

pub async fn get_dialog_ids(pool: &SqlitePool, user_id: i64) -> Result<Vec<i32>> {
    let dialog_ids = sqlx::query_scalar(
        "SELECT dialog_id FROM user_dialogs WHERE user_id = ? ORDER BY dialog_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(dialog_ids)
}

pub async fn add_dialog(pool: &SqlitePool, user_id: i64, dialog_id: i32) -> Result<()> {
    sqlx::query(
        "INSERT INTO user_dialogs (user_id, dialog_id) VALUES (?, ?) ON CONFLICT DO NOTHING",
    )
    .bind(user_id)
    .bind(dialog_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn has_dialog(pool: &SqlitePool, user_id: i64, dialog_id: i32) -> Result<bool> {
    let exists: Option<i32> =
        sqlx::query_scalar("SELECT 1 FROM user_dialogs WHERE user_id = ? AND dialog_id = ?")
            .bind(user_id)
            .bind(dialog_id)
            .fetch_optional(pool)
            .await?;
    Ok(exists.is_some())
}
