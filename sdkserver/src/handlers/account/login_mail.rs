use crate::AppState;
use crate::models::request::AccountLoginMailReq;
use crate::models::response::{AccountLoginRsp, AccountLoginRspData, AccountType, RealNameInfo};
use axum::{extract::State, response::Json};
use common::time::ServerTime;
use database::db::user::account::{TokenInfo, handle_user_login};

pub async fn post(
    State(state): State<AppState>,
    axum::Json(req): axum::Json<AccountLoginMailReq>,
) -> Json<AccountLoginRsp> {
    let now = ServerTime::now_ms() as i64;

    tracing::info!(
        "Login attempt - Email: {}, Device: {}, OS: {}",
        req.account,
        req.device_info.device_name,
        req.device_info.os_version
    );

    // Generate tokens
    let token = generate_token();
    let refresh_token = generate_token();
    let expires_in = 7 * 24 * 60 * 60; // 7 days in seconds
    let token_expires_at = now + (expires_in * 1000);

    let token_info = TokenInfo {
        token: token.clone(),
        refresh_token: refresh_token.clone(),
        expires_at: token_expires_at,
    };

    // Handle login with password verification
    let user = match handle_user_login(
        &state.game.db,
        &req.account, // Email
        &req.pwd,     // Password hash from client
        token_info,
        now,
    )
    .await
    {
        Ok(user) => user,
        Err(e) => {
            tracing::warn!("Login failed for {}: {}", req.account, e);
            return Json(create_error_response());
        }
    };

    tracing::info!(
        "Login successful - User ID: {}, Email: {}, First join: {}",
        user.id,
        req.account,
        user.first_join
    );

    let rsp = AccountLoginRsp {
        code: 200,
        msg: "success".to_string(),
        data: AccountLoginRspData {
            token,
            expires_in,
            refresh_token,
            user_id: user.id as u64, // Use actual user ID from database
            account_type: AccountType::Email,
            registration_account_type: 1,
            account: mask_email(&user.email), // Mask the email from database
            real_name_info: RealNameInfo {
                need_real_name: user.need_real_name,
                real_name_status: user.real_name_status,
                age: user.age as u8,
                adult: user.is_adult,
            },
            need_activate: user.need_activate,
            cipher_mark: user.cipher_mark,
            first_join: user.first_join,
            account_tags: user.account_tags,
        },
    };

    Json(rsp)
}

fn mask_email(email: &str) -> String {
    if let Some(at_pos) = email.find('@') {
        let (local, domain) = email.split_at(at_pos);
        let prefix_len = local.chars().count().min(3);
        format!(
            "{}****{}",
            local.chars().take(prefix_len).collect::<String>(),
            domain
        )
    } else {
        email.to_string()
    }
}

fn generate_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 16];
    rng.fill(&mut bytes);
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
        + "200"
}

fn create_error_response() -> AccountLoginRsp {
    AccountLoginRsp {
        code: 401,
        msg: "Invalid email or password".to_string(),
        data: AccountLoginRspData {
            token: String::new(),
            expires_in: 0,
            refresh_token: String::new(),
            user_id: 0,
            account_type: AccountType::Email,
            registration_account_type: 0,
            account: String::new(),
            real_name_info: RealNameInfo {
                need_real_name: false,
                real_name_status: false,
                age: 0,
                adult: false,
            },
            need_activate: false,
            cipher_mark: false,
            first_join: false,
            account_tags: String::new(),
        },
    }
}
