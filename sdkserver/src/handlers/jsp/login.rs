use crate::AppState;
use crate::models::request::JspLoginQuery;
use crate::models::response::{JspLoginRsp, VerifyUserInfo, ZoneInfo};
use axum::{
    extract::{Query, State},
    response::Json,
};
use sqlx::Row;

pub async fn get(
    State(state): State<AppState>,
    Query(params): Query<JspLoginQuery>,
) -> Json<JspLoginRsp> {
    tracing::info!("JSP login request - Account ID: {}", params.account_id);

    // Extract user_id from accountId format: "200_1337" -> 1337
    let user_id: u64 = params
        .account_id
        .split('_')
        .last()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    if user_id == 0 {
        tracing::error!("Invalid accountId format: {}", params.account_id);
        return Json(JspLoginRsp {
            result_code: 1,
            ..Default::default()
        });
    }

    tracing::info!("Extracted user_id: {}", user_id);

    // Fetch token and account_tags from database
    let user = sqlx::query("SELECT token, account_tags FROM users WHERE id = ?1")
        .bind(user_id as i64)
        .fetch_optional(&state.game.db)
        .await
        .ok()
        .flatten();

    match user {
        Some(row) => {
            let token: String = row.try_get("token").unwrap_or_else(|_| {
                tracing::error!("No token for user {}", user_id);
                "invalid-token".to_string()
            });

            let account_tags: String = row
                .try_get::<Option<String>, _>("account_tags")
                .ok()
                .flatten()
                .unwrap_or_default();

            tracing::info!("JSP login successful for user {}", user_id);

            let rsp = JspLoginRsp {
                account_tags,
                area_id: 4,
                is_admin: false,
                result_code: 0,
                session_id: token, // Real token from database
                user_name: VerifyUserInfo::user_id(user_id), // Use helper: "200_1337"
                zone_info: ZoneInfo::zone_four(),
            };

            Json(rsp)
        }
        None => {
            tracing::warn!("User {} not found in database", user_id);
            Json(JspLoginRsp {
                result_code: 1,
                session_id: String::new(),
                ..Default::default()
            })
        }
    }
}
