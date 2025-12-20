use super::helpers::*;
use crate::AppState;
use crate::models::request::AccountBindListReq;
use crate::models::response::{AccountBindListRsp, AccountBindListRspData, AccountType};
use axum::{extract::State, response::Json};

pub async fn post(
    State(state): State<AppState>,
    axum::Json(req): axum::Json<AccountBindListReq>,
) -> Json<AccountBindListRsp> {
    let user = match get_user_by_id(&state, req.user_id as i64).await {
        Ok(user) => user,
        Err(_) => {
            return Json(AccountBindListRsp {
                code: 200,
                msg: "success".to_string(),
                data: vec![],
            });
        }
    };

    Json(AccountBindListRsp {
        code: 200,
        msg: "success".to_string(),
        data: vec![AccountBindListRspData {
            user_id: req.user_id,
            account: mask_email(&user.email),
            account_type: AccountType::Email,
        }],
    })
}
