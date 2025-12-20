use anyhow::Result;
use sqlx::SqlitePool;

pub async fn save_round_operations(
    pool: &SqlitePool,
    user_id: i64,
    episode_id: i32,
    battle_id: i64,
    round_number: i32,
    cloth_skill_opers: Vec<sonettobuf::UseClothSkillOperRecord>,
    opers: Vec<sonettobuf::BeginRoundOper>,
) -> Result<()> {
    let cloth_json = serde_json::to_string(&cloth_skill_opers)?;
    let opers_json = serde_json::to_string(&opers)?;

    sqlx::query(
        "INSERT OR REPLACE INTO battle_replays
         (user_id, episode_id, battle_id, round_number, cloth_skill_opers, opers, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(user_id)
    .bind(episode_id)
    .bind(battle_id)
    .bind(round_number)
    .bind(cloth_json)
    .bind(opers_json)
    .bind(chrono::Utc::now().timestamp())
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn load_battle_replay(
    pool: &SqlitePool,
    user_id: i64,
    episode_id: i32,
) -> Result<Vec<sonettobuf::FightRoundOperRecord>> {
    #[allow(dead_code)]
    #[derive(sqlx::FromRow)]
    struct ReplayRow {
        round_number: i32,
        cloth_skill_opers: String,
        opers: String,
    }

    let rows: Vec<ReplayRow> = sqlx::query_as(
        "SELECT round_number, cloth_skill_opers, opers
         FROM battle_replays
         WHERE user_id = ? AND episode_id = ?
         ORDER BY round_number",
    )
    .bind(user_id)
    .bind(episode_id)
    .fetch_all(pool)
    .await?;

    let mut records = Vec::new();
    for row in rows {
        let cloth_skill_opers: Vec<sonettobuf::UseClothSkillOperRecord> =
            serde_json::from_str(&row.cloth_skill_opers)?;
        let opers: Vec<sonettobuf::BeginRoundOper> = serde_json::from_str(&row.opers)?;

        records.push(sonettobuf::FightRoundOperRecord {
            cloth_skill_opers,
            opers,
        });
    }

    Ok(records)
}
