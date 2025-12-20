use crate::AppState;
use crate::models::request::LoadZoneQuery;
use crate::models::response::{JspLoadZoneRsp, ZoneInfo, ZoneUserInfo};
use axum::{
    extract::{Query, State},
    response::Json,
};

use sqlx::Row;

pub async fn get(
    State(state): State<AppState>,
    Query(params): Query<LoadZoneQuery>,
) -> Json<JspLoadZoneRsp> {
    tracing::info!(
        "LoadZone request - Token: {}...",
        &params.session_id[..8.min(params.session_id.len())]
    );

    // Look up user by token
    let user = sqlx::query("SELECT id, username, level FROM users WHERE token = ?1")
        .bind(&params.session_id)
        .fetch_optional(&state.game.db)
        .await
        .ok()
        .flatten();

    match user {
        Some(row) => {
            let user_id: i64 = row.try_get("id").unwrap_or(0);
            let username: String = row
                .try_get("username")
                .unwrap_or_else(|_| "Player".to_string());
            let level: i64 = row.try_get("level").unwrap_or(1);

            tracing::info!("LoadZone successful for user {} ({})", user_id, username);

            let rsp = JspLoadZoneRsp {
                last_login_zone_id: 4,
                recommend_zone_id: 4,
                result_code: 0,
                user_infos: vec![ZoneUserInfo {
                    id: 4, // zone_id
                    level: level as i32,
                    name: username,
                    portrait: 171504, // Default portrait
                }],
                zone_infos: vec![ZoneInfo::zone_four()],
            };

            Json(rsp)
        }
        None => {
            tracing::warn!("Invalid token in LoadZone request");
            Json(JspLoadZoneRsp {
                last_login_zone_id: 4,
                recommend_zone_id: 4,
                result_code: 1, // Error
                user_infos: vec![],
                zone_infos: vec![ZoneInfo::zone_four()],
            })
        }
    }
}
