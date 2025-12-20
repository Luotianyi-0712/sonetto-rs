use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use database::db::game::bgm::set_active_bgm;
use prost::Message;
use sonettobuf::{CmdId, SetUseBgmReply, SetUseBgmRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_set_use_bgm(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = SetUseBgmRequest::decode(&req.data[..])?;

    let mut ctx_guard = ctx.lock().await;
    let player_id = ctx_guard.player_id.ok_or(AppError::NotLoggedIn)?;
    let pool = &ctx_guard.state.db;

    set_active_bgm(pool, player_id, request.bgm_id.unwrap_or(2207))
        .await
        .map_err(AppError::from)?;

    let reply = SetUseBgmReply {
        bgm_id: request.bgm_id,
    };

    ctx_guard
        .send_reply(CmdId::SetUseBgmCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
