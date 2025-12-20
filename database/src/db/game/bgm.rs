use crate::models::game::bgm::{UserBgm, UserBgmState};
use anyhow::Result;
use sonettobuf::BgmInfo;
use sqlx::SqlitePool;

async fn load_bgms(pool: &SqlitePool, player_id: i64) -> Result<Vec<UserBgm>> {
    Ok(sqlx::query_as::<_, UserBgm>(
        r#"
            SELECT
                player_id,
                bgm_id,
                unlock_time,
                is_favorite,
                is_read
            FROM user_bgm
            WHERE player_id = ?
            ORDER BY unlock_time
            "#,
    )
    .bind(player_id)
    .fetch_all(pool)
    .await?)
}

async fn load_bgm_state(pool: &SqlitePool, player_id: i64) -> Result<Option<UserBgmState>> {
    Ok(sqlx::query_as::<_, UserBgmState>(
        r#"
            SELECT player_id, use_bgm_id
            FROM user_bgm_state
            WHERE player_id = ?
            "#,
    )
    .bind(player_id)
    .fetch_optional(pool)
    .await?)
}

pub async fn load_user_bgm(
    pool: &SqlitePool,
    player_id: i64,
) -> Result<(Vec<BgmInfo>, Option<i32>)> {
    let bgms = load_bgms(pool, player_id).await?;
    let state = load_bgm_state(pool, player_id).await?;

    Ok((
        bgms.into_iter().map(Into::into).collect(),
        state.map(|s| s.use_bgm_id),
    ))
}

pub async fn set_active_bgm(pool: &SqlitePool, player_id: i64, bgm_id: i32) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    let old_bgm: Option<i32> =
        sqlx::query_scalar("SELECT use_bgm_id FROM user_bgm_state WHERE player_id = ?")
            .bind(player_id)
            .fetch_optional(&mut *tx)
            .await?;

    if let Some(old) = old_bgm {
        if old != bgm_id {
            sqlx::query("UPDATE user_bgm SET is_favorite = 0 WHERE player_id = ? AND bgm_id = ?")
                .bind(player_id)
                .bind(old)
                .execute(&mut *tx)
                .await?;
        }
    }

    sqlx::query(
        "UPDATE user_bgm SET is_favorite = 1, is_read = 1 WHERE player_id = ? AND bgm_id = ?",
    )
    .bind(player_id)
    .bind(bgm_id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO user_bgm_state (player_id, use_bgm_id)
        VALUES (?, ?)
        ON CONFLICT(player_id)
        DO UPDATE SET use_bgm_id = excluded.use_bgm_id
        "#,
    )
    .bind(player_id)
    .bind(bgm_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn set_bgm_favorite(
    pool: &SqlitePool,
    player_id: i64,
    bgm_id: i32,
    favorite: bool,
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        UPDATE user_bgm
        SET is_favorite = ?, is_read = 1
        WHERE player_id = ? AND bgm_id = ?
        "#,
    )
    .bind(favorite)
    .bind(player_id)
    .bind(bgm_id)
    .execute(pool)
    .await?;

    Ok(())
}
