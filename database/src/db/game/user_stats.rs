use crate::models::game::user_stats::UserStats;
use sqlx::SqlitePool;

pub async fn get_user_stats(pool: &SqlitePool, user_id: i64) -> sqlx::Result<Option<UserStats>> {
    sqlx::query_as::<_, UserStats>(
        "SELECT user_id, first_charge, total_charge_amount, is_first_login, user_tag
         FROM user_stats
         WHERE user_id = ?",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn update_first_charge(pool: &SqlitePool, user_id: i64, amount: i64) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE user_stats
         SET first_charge = 1,
             total_charge_amount = total_charge_amount + ?
         WHERE user_id = ?",
    )
    .bind(amount)
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn add_charge_amount(pool: &SqlitePool, user_id: i64, amount: i64) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE user_stats
         SET total_charge_amount = total_charge_amount + ?
         WHERE user_id = ?",
    )
    .bind(amount)
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_not_first_login(pool: &SqlitePool, user_id: i64) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE user_stats
         SET is_first_login = 0
         WHERE user_id = ?",
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_user_tag(pool: &SqlitePool, user_id: i64, tag: &str) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE user_stats
         SET user_tag = ?
         WHERE user_id = ?",
    )
    .bind(tag)
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}
