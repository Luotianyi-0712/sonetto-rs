use super::helpers::*;
use crate::AppState;
use crate::models::request::AccountLoginVerifyReq;
use crate::models::response::{AccountLoginVerifyRsp, AccountLoginVerifyRspData, VerifyUserInfo};
use axum::{extract::State, response::Json};

pub async fn post(
    State(state): State<AppState>,
    axum::Json(req): axum::Json<AccountLoginVerifyReq>,
) -> Json<AccountLoginVerifyRsp> {
    // Parse user_id from string
    let user_id: i64 = match req.user_id.parse() {
        Ok(id) => id,
        Err(_) => {
            tracing::error!("Invalid user_id format: {}", req.user_id);
            return Json(create_verify_error_response());
        }
    };

    tracing::debug!(
        "Login verify request - User ID: {}, Token: {}...",
        user_id,
        &req.token[..8]
    );

    // Validate token and get user
    let user = match get_user_with_token_validation(&state, user_id, &req.token).await {
        Ok(user) => user,
        Err(e) => {
            tracing::warn!("Login verify failed: {}", e);
            return Json(create_verify_error_response());
        }
    };

    let expires_in = calculate_expires_in(user.token_expires_at);
    let register_time = format_timestamp(user.created_at);
    let first_join_time = format_timestamp(user.last_login_at);

    tracing::info!("Login verify successful for user {}", user_id);

    let rsp = AccountLoginVerifyRsp {
        code: 200,
        msg: "success".to_string(),
        data: AccountLoginVerifyRspData {
            user_info: VerifyUserInfo {
                channel_id: 200,
                user_id: req.user_id, // Return as string (same as request)
                real_name_status: user.real_name_status,
                age: user.age,
                adult: user.is_adult,
                account_tags: user.account_tags,
                bind_account_type_list: vec![user.email],
                first_join_time,
                register_time,
                is_pay_account: true,
                first_join: user.first_join,
            },
            session_id: generate_session_id(),
            token: user.token,
            expires_in,
            refresh_token: user.refresh_token,
        },
    };

    Json(rsp)
}

fn create_verify_error_response() -> AccountLoginVerifyRsp {
    AccountLoginVerifyRsp {
        code: 401,
        msg: "Invalid token or user not found".to_string(),
        data: AccountLoginVerifyRspData {
            user_info: VerifyUserInfo {
                channel_id: 200,
                user_id: String::new(),
                real_name_status: false,
                age: 0,
                adult: false,
                account_tags: String::new(),
                bind_account_type_list: vec![],
                first_join_time: String::new(),
                register_time: String::new(),
                is_pay_account: false,
                first_join: false,
            },
            session_id: String::new(),
            token: String::new(),
            expires_in: 0,
            refresh_token: String::new(),
        },
    }
}
