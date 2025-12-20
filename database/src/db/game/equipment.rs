use anyhow::Result;
use sonettobuf::FightEquipRecord;
use sqlx::SqlitePool;

pub use crate::models::game::equipment::Equipment;

/// Get all equipment for a user
pub async fn get_user_equipment(pool: &SqlitePool, user_id: i64) -> Result<Vec<Equipment>> {
    let equipment = sqlx::query_as::<_, Equipment>(
        "SELECT * FROM equipment WHERE user_id = ?1 ORDER BY equip_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(equipment)
}

/// Create equipment for user
pub async fn create_equipment(
    pool: &SqlitePool,
    uid: i64,
    user_id: i64,
    equip_id: i32,
    level: i32,
    break_lv: i32,
    refine_lv: i32,
    now: i64,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO equipment (
            uid, user_id, equip_id, level, exp, break_lv, count, is_lock, refine_lv, created_at, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
        "#,
    )
    .bind(uid)
    .bind(user_id)
    .bind(equip_id)
    .bind(level)
    .bind(0) // exp
    .bind(break_lv)
    .bind(1) // count
    .bind(false) // is_lock
    .bind(refine_lv)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_hero_default_equip_id(
    pool: &SqlitePool,
    hero_uid: i64,
    user_id: i64,
) -> Result<Option<i32>> {
    let equip_id: Option<i32> = sqlx::query_scalar(
        r#"
        SELECT e.equip_id
        FROM heroes h
        LEFT JOIN equipment e
          ON e.uid = h.default_equip_uid
        WHERE h.uid = ? AND h.user_id = ?
        "#,
    )
    .bind(hero_uid)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(equip_id)
}

pub async fn get_equipment_by_uid(
    pool: &SqlitePool,
    user_id: i64,
    equip_uid: i64,
) -> Result<Equipment> {
    let equip = sqlx::query_as::<_, Equipment>(
        r#"
        SELECT uid, user_id, equip_id, level, exp, break_lv, count, is_lock, refine_lv, created_at, updated_at
        FROM equipment
        WHERE uid = ? AND user_id = ?
        "#,
    )
    .bind(equip_uid)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(equip)
}

pub async fn build_equip_records(
    pool: &SqlitePool,
    player_id: i64,
    fight_group: &Option<sonettobuf::FightGroup>,
) -> Result<Vec<FightEquipRecord>> {
    let Some(fg) = fight_group else {
        return Ok(vec![]);
    };

    let mut equip_records = Vec::new();

    for equip in &fg.equips {
        let hero_uid = equip.hero_uid.unwrap_or(0);
        let mut records = Vec::new();

        for &equip_uid in &equip.equip_uid {
            if equip_uid == 0 {
                continue;
            }

            // Load equipment details
            if let Ok(equip_data) = get_equipment_by_uid(pool, player_id, equip_uid).await {
                records.push(sonettobuf::EquipRecord {
                    equip_uid: Some(equip_uid),
                    equip_id: Some(equip_data.equip_id),
                    equip_lv: Some(equip_data.level),
                    refine_lv: Some(equip_data.refine_lv),
                });
            }
        }

        equip_records.push(FightEquipRecord {
            hero_uid: Some(hero_uid),
            equip_records: records,
        });
    }

    Ok(equip_records)
}
