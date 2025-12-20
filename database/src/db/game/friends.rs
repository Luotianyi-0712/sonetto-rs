use anyhow::Result;
use sqlx::SqlitePool;

pub async fn get_friend_ids(pool: &SqlitePool, user_id: i64) -> Result<Vec<u64>> {
    let friends = sqlx::query_scalar(
        "SELECT friend_id FROM user_friends WHERE user_id = ? ORDER BY created_at",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(friends.into_iter().map(|id: i64| id as u64).collect())
}

pub async fn get_blacklist_ids(pool: &SqlitePool, user_id: i64) -> Result<Vec<u64>> {
    let blacklist = sqlx::query_scalar(
        "SELECT blocked_user_id FROM user_blacklist WHERE user_id = ? ORDER BY created_at",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(blacklist.into_iter().map(|id: i64| id as u64).collect())
}

pub async fn add_friend(pool: &SqlitePool, user_id: i64, friend_id: i64) -> Result<()> {
    let now = chrono::Utc::now().timestamp();

    sqlx::query(
        "INSERT INTO user_friends (user_id, friend_id, created_at) VALUES (?, ?, ?) ON CONFLICT DO NOTHING"
    )
    .bind(user_id)
    .bind(friend_id)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_friend(pool: &SqlitePool, user_id: i64, friend_id: i64) -> Result<()> {
    sqlx::query("DELETE FROM user_friends WHERE user_id = ? AND friend_id = ?")
        .bind(user_id)
        .bind(friend_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn add_to_blacklist(pool: &SqlitePool, user_id: i64, blocked_id: i64) -> Result<()> {
    let now = chrono::Utc::now().timestamp();

    sqlx::query(
        "INSERT INTO user_blacklist (user_id, blocked_user_id, created_at) VALUES (?, ?, ?) ON CONFLICT DO NOTHING"
    )
    .bind(user_id)
    .bind(blocked_id)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_from_blacklist(pool: &SqlitePool, user_id: i64, blocked_id: i64) -> Result<()> {
    sqlx::query("DELETE FROM user_blacklist WHERE user_id = ? AND blocked_user_id = ?")
        .bind(user_id)
        .bind(blocked_id)
        .execute(pool)
        .await?;

    Ok(())
}
