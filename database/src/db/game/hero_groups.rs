use crate::models::game::hero_groups::{
    HeroGroupCommon, HeroGroupEquip, HeroGroupInfo, HeroGroupType, HeroGroupTypeInfo,
};
use anyhow::Result;
use sqlx::SqlitePool;
use std::collections::HashMap;

/// Helper to build HeroGroupInfo from a group_id
async fn build_hero_group_info(
    pool: &SqlitePool,
    _user_id: i64,
    db_group_id: i64,
    group_id: i32,
) -> Result<HeroGroupInfo> {
    // Get hero members
    let hero_list: Vec<i64> = sqlx::query_scalar(
        "SELECT hero_uid FROM hero_group_members WHERE hero_group_id = ? ORDER BY position",
    )
    .bind(db_group_id)
    .fetch_all(pool)
    .await?;

    // Get group details
    let group =
        sqlx::query_as::<_, HeroGroupCommon>("SELECT * FROM hero_groups_common WHERE id = ?")
            .bind(db_group_id)
            .fetch_one(pool)
            .await?;

    // Get equips
    let equip_rows: Vec<(i32, i64)> = sqlx::query_as(
        "SELECT index_slot, equip_uid FROM hero_group_equips WHERE hero_group_id = ? ORDER BY index_slot"
    )
    .bind(db_group_id)
    .fetch_all(pool)
    .await?;

    let mut equips_map: HashMap<i32, Vec<i64>> = HashMap::new();
    for (index, equip_uid) in equip_rows {
        equips_map.entry(index).or_default().push(equip_uid);
    }

    let equips = equips_map
        .into_iter()
        .map(|(index, equip_uids)| HeroGroupEquip { index, equip_uids })
        .collect();

    // Get activity104 equips
    let activity104_rows: Vec<(i32, i64)> = sqlx::query_as(
        "SELECT index_slot, equip_uid FROM hero_group_activity104_equips WHERE hero_group_id = ? ORDER BY index_slot"
    )
    .bind(db_group_id)
    .fetch_all(pool)
    .await?;

    let mut activity104_map: HashMap<i32, Vec<i64>> = HashMap::new();
    for (index, equip_uid) in activity104_rows {
        activity104_map.entry(index).or_default().push(equip_uid);
    }

    let activity104_equips = activity104_map
        .into_iter()
        .map(|(index, equip_uids)| HeroGroupEquip { index, equip_uids })
        .collect();

    Ok(HeroGroupInfo {
        group_id,
        hero_list,
        name: group.name,
        cloth_id: group.cloth_id,
        equips,
        activity104_equips,
        assist_boss_id: group.assist_boss_id,
    })
}

/// Get ONE specific hero group (for GetHeroGroupList - returns current active group)
pub async fn get_hero_group(
    pool: &SqlitePool,
    user_id: i64,
    group_id: i32,
) -> Result<Option<HeroGroupInfo>> {
    // Find the DB id for this group_id
    let db_group_id: Option<i64> =
        sqlx::query_scalar("SELECT id FROM hero_groups_common WHERE user_id = ? AND group_id = ?")
            .bind(user_id)
            .bind(group_id)
            .fetch_optional(pool)
            .await?;

    if let Some(db_id) = db_group_id {
        Ok(Some(
            build_hero_group_info(pool, user_id, db_id, group_id).await?,
        ))
    } else {
        Ok(None)
    }
}

/// Get current active group (probably type 1's current selection)
pub async fn get_current_hero_group(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Option<HeroGroupInfo>> {
    // Get the current selected group from type 1 (main battle group)
    let selected_group: Option<i32> = sqlx::query_scalar(
        "SELECT group_id FROM hero_group_types WHERE user_id = ? AND type_id = 1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    if let Some(group_id) = selected_group {
        get_hero_group(pool, user_id, group_id).await
    } else {
        Ok(None)
    }
}

/// Get ALL common hero groups (for GetHeroGroupCommonList)
pub async fn get_hero_groups_common(pool: &SqlitePool, user_id: i64) -> Result<Vec<HeroGroupInfo>> {
    let groups = sqlx::query_as::<_, HeroGroupCommon>(
        "SELECT * FROM hero_groups_common WHERE user_id = ? ORDER BY group_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();
    for group in groups {
        let info = build_hero_group_info(pool, user_id, group.id, group.group_id).await?;
        result.push(info);
    }

    Ok(result)
}

/// Get all hero group types (for GetHeroGroupCommonList)
pub async fn get_hero_group_types(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<HeroGroupTypeInfo>> {
    let types = sqlx::query_as::<_, HeroGroupType>(
        "SELECT * FROM hero_group_types WHERE user_id = ? ORDER BY type_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();
    for type_info in types {
        let group_info = if let Some(group_id) = type_info.group_id {
            get_hero_group(pool, user_id, group_id).await?
        } else {
            None
        };

        result.push(HeroGroupTypeInfo {
            type_id: type_info.type_id,
            current_select: type_info.current_select,
            group_info,
        });
    }

    Ok(result)
}

/// Set equipment for a hero group
pub async fn set_hero_group_equip(
    pool: &SqlitePool,
    user_id: i64,
    group_id: i32,
    index: i32,
    equip_uids: Vec<i64>,
) -> Result<()> {
    // Get the DB group ID
    let db_group_id: Option<i64> =
        sqlx::query_scalar("SELECT id FROM hero_groups_common WHERE user_id = ? AND group_id = ?")
            .bind(user_id)
            .bind(group_id)
            .fetch_optional(pool)
            .await?;

    let db_group_id = db_group_id.ok_or_else(|| anyhow::anyhow!("Hero group not found"))?;

    // Delete existing equips for this index
    sqlx::query("DELETE FROM hero_group_equips WHERE hero_group_id = ? AND index_slot = ?")
        .bind(db_group_id)
        .bind(index)
        .execute(pool)
        .await?;

    // Insert new equips
    for equip_uid in equip_uids {
        sqlx::query(
            "INSERT INTO hero_group_equips (hero_group_id, index_slot, equip_uid) VALUES (?, ?, ?)",
        )
        .bind(db_group_id)
        .bind(index)
        .bind(equip_uid)
        .execute(pool)
        .await?;
    }

    Ok(())
}
