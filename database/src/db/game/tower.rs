use crate::models::game::tower::*;
use anyhow::Result;
use sonettobuf;
use sqlx::SqlitePool;

pub async fn get_tower_info(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<(
    UserTowerInfo,
    Vec<TowerOpen>,
    Vec<sonettobuf::TowerNo>,
    Vec<AssistBossInfo>,
)> {
    // Get main info
    let info =
        sqlx::query_as::<_, UserTowerInfo>("SELECT * FROM user_tower_info WHERE user_id = ?")
            .bind(user_id)
            .fetch_optional(pool)
            .await?
            .unwrap_or(UserTowerInfo {
                user_id,
                mop_up_times: 0,
                trial_hero_season: 0,
            });

    // Get tower opens
    let tower_opens = sqlx::query_as::<_, TowerOpen>(
        "SELECT tower_type, tower_id, status, round, next_time, tower_start_time, task_end_time
         FROM user_tower_opens WHERE user_id = ? ORDER BY tower_type, tower_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    // Get towers
    let towers = get_towers(pool, user_id).await?;

    // Get assist bosses
    let assist_bosses = get_assist_bosses(pool, user_id).await?;

    Ok((info, tower_opens, towers, assist_bosses))
}

async fn get_towers(pool: &SqlitePool, user_id: i64) -> Result<Vec<sonettobuf::TowerNo>> {
    let tower_data: Vec<(i32, i32, i32, i32, String)> = sqlx::query_as(
        "SELECT tower_type, tower_id, pass_layer_id, history_high_score, params
         FROM user_towers WHERE user_id = ? ORDER BY tower_type, tower_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut towers = Vec::new();
    for (tower_type, tower_id, pass_layer_id, history_high_score, params) in tower_data {
        // Get layers
        let layer_nos = get_tower_layers(pool, user_id, tower_type, tower_id).await?;

        // Get open special layer IDs
        let open_sp_layer_ids = sqlx::query_scalar(
            "SELECT sp_layer_id FROM user_tower_open_sp_layers WHERE user_id = ? AND tower_type = ? AND tower_id = ?"
        )
        .bind(user_id)
        .bind(tower_type)
        .bind(tower_id)
        .fetch_all(pool)
        .await?;

        // Get pass teach IDs
        let pass_teach_ids = sqlx::query_scalar(
            "SELECT teach_id FROM user_tower_pass_teaches WHERE user_id = ? AND tower_type = ? AND tower_id = ?"
        )
        .bind(user_id)
        .bind(tower_type)
        .bind(tower_id)
        .fetch_all(pool)
        .await?;

        towers.push(sonettobuf::TowerNo {
            r#type: Some(tower_type),
            tower_id: Some(tower_id),
            pass_layer_id: Some(pass_layer_id),
            layer_n_os: layer_nos,
            open_sp_layer_ids,
            history_high_score: Some(history_high_score),
            params: Some(params),
            pass_teach_ids,
        });
    }

    Ok(towers)
}

async fn get_tower_layers(
    pool: &SqlitePool,
    user_id: i64,
    tower_type: i32,
    tower_id: i32,
) -> Result<Vec<sonettobuf::LayerNo>> {
    let layers: Vec<(i32, i32, i32)> = sqlx::query_as(
        "SELECT layer_id, curr_high_score, history_high_score
         FROM user_tower_layers WHERE user_id = ? AND tower_type = ? AND tower_id = ?
         ORDER BY layer_id",
    )
    .bind(user_id)
    .bind(tower_type)
    .bind(tower_id)
    .fetch_all(pool)
    .await?;

    let mut layer_nos = Vec::new();
    for (layer_id, curr_high_score, history_high_score) in layers {
        let episode_nos = get_layer_episodes(pool, user_id, tower_type, tower_id, layer_id).await?;

        layer_nos.push(sonettobuf::LayerNo {
            layer_id: Some(layer_id),
            curr_high_score: Some(curr_high_score),
            history_high_score: Some(history_high_score),
            episode_n_os: episode_nos,
        });
    }

    Ok(layer_nos)
}

async fn get_layer_episodes(
    pool: &SqlitePool,
    user_id: i64,
    tower_type: i32,
    tower_id: i32,
    layer_id: i32,
) -> Result<Vec<sonettobuf::EpisodeNo>> {
    let episodes: Vec<(i32, i32, i32)> = sqlx::query_as(
        "SELECT episode_id, status, assist_boss_id
         FROM user_tower_episodes
         WHERE user_id = ? AND tower_type = ? AND tower_id = ? AND layer_id = ?
         ORDER BY episode_id",
    )
    .bind(user_id)
    .bind(tower_type)
    .bind(tower_id)
    .bind(layer_id)
    .fetch_all(pool)
    .await?;

    let mut episode_nos = Vec::new();
    for (episode_id, status, assist_boss_id) in episodes {
        let heroes =
            get_episode_heroes(pool, user_id, tower_type, tower_id, layer_id, episode_id).await?;

        episode_nos.push(sonettobuf::EpisodeNo {
            episode_id: Some(episode_id),
            status: Some(status),
            heros: heroes.into_iter().map(Into::into).collect(),
            assist_boss_id: Some(assist_boss_id),
        });
    }

    Ok(episode_nos)
}

async fn get_episode_heroes(
    pool: &SqlitePool,
    user_id: i64,
    tower_type: i32,
    tower_id: i32,
    layer_id: i32,
    episode_id: i32,
) -> Result<Vec<HeroInfo>> {
    let hero_data: Vec<(i32, i32)> = sqlx::query_as(
        "SELECT hero_id, trial_id FROM user_tower_episode_heroes
         WHERE user_id = ? AND tower_type = ? AND tower_id = ? AND layer_id = ? AND episode_id = ?",
    )
    .bind(user_id)
    .bind(tower_type)
    .bind(tower_id)
    .bind(layer_id)
    .bind(episode_id)
    .fetch_all(pool)
    .await?;

    let mut heroes = Vec::new();
    for (hero_id, trial_id) in hero_data {
        let equip_uids = sqlx::query_scalar(
            "SELECT equip_uid FROM user_tower_episode_hero_equips
             WHERE user_id = ? AND tower_type = ? AND tower_id = ? AND layer_id = ? AND episode_id = ? AND hero_id = ?"
        )
        .bind(user_id)
        .bind(tower_type)
        .bind(tower_id)
        .bind(layer_id)
        .bind(episode_id)
        .bind(hero_id)
        .fetch_all(pool)
        .await?;

        heroes.push(HeroInfo {
            hero_id,
            equip_uids,
            trial_id,
        });
    }

    Ok(heroes)
}

async fn get_assist_bosses(pool: &SqlitePool, user_id: i64) -> Result<Vec<AssistBossInfo>> {
    let bosses: Vec<(i32, i32, i32)> = sqlx::query_as(
        "SELECT boss_id, level, use_talent_plan FROM user_assist_bosses WHERE user_id = ? ORDER BY boss_id"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut assist_bosses = Vec::new();
    for (boss_id, level, use_talent_plan) in bosses {
        // Get talent plans
        let plan_data: Vec<(i32, i32, String)> = sqlx::query_as(
            "SELECT plan_id, talent_point, plan_name
             FROM user_assist_boss_talent_plans WHERE user_id = ? AND boss_id = ? ORDER BY plan_id",
        )
        .bind(user_id)
        .bind(boss_id)
        .fetch_all(pool)
        .await?;

        let mut talent_plans = Vec::new();
        for (plan_id, talent_point, plan_name) in plan_data {
            let talent_ids = sqlx::query_scalar(
                "SELECT talent_id FROM user_assist_boss_plan_talents WHERE user_id = ? AND boss_id = ? AND plan_id = ?"
            )
            .bind(user_id)
            .bind(boss_id)
            .bind(plan_id)
            .fetch_all(pool)
            .await?;

            talent_plans.push(TalentPlanInfo {
                plan_id,
                talent_point,
                talent_ids,
                plan_name,
            });
        }

        assist_bosses.push(AssistBossInfo {
            boss_id,
            level,
            talent_plans,
            use_talent_plan,
        });
    }

    Ok(assist_bosses)
}

/// Update tower layer score after battle
pub async fn update_tower_layer_score(
    pool: &SqlitePool,
    user_id: i64,
    tower_type: i32,
    tower_id: i32,
    layer_id: i32,
    score: i32,
) -> sqlx::Result<()> {
    // Update current and history high scores
    sqlx::query(
        "UPDATE user_tower_layers
         SET curr_high_score = ?,
             history_high_score = MAX(history_high_score, ?)
         WHERE user_id = ? AND tower_type = ? AND tower_id = ? AND layer_id = ?",
    )
    .bind(score)
    .bind(score)
    .bind(user_id)
    .bind(tower_type)
    .bind(tower_id)
    .bind(layer_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Update tower episode status
pub async fn update_tower_episode_status(
    pool: &SqlitePool,
    user_id: i64,
    tower_type: i32,
    tower_id: i32,
    layer_id: i32,
    episode_id: i32,
    status: i32,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE user_tower_episodes
         SET status = ?
         WHERE user_id = ? AND tower_type = ? AND tower_id = ? AND layer_id = ? AND episode_id = ?",
    )
    .bind(status)
    .bind(user_id)
    .bind(tower_type)
    .bind(tower_id)
    .bind(layer_id)
    .bind(episode_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Update tower pass layer
pub async fn update_tower_pass_layer(
    pool: &SqlitePool,
    user_id: i64,
    tower_type: i32,
    tower_id: i32,
    pass_layer_id: i32,
    history_high_score: i32,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE user_towers
         SET pass_layer_id = MAX(pass_layer_id, ?),
             history_high_score = MAX(history_high_score, ?)
         WHERE user_id = ? AND tower_type = ? AND tower_id = ?",
    )
    .bind(pass_layer_id)
    .bind(history_high_score)
    .bind(user_id)
    .bind(tower_type)
    .bind(tower_id)
    .execute(pool)
    .await?;

    Ok(())
}
