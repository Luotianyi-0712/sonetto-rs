use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use database::db::user::account::rename_user_and_update_guide;
use prost::Message;
use sonettobuf::{CmdId, RenameReply, RenameRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_rename(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = RenameRequest::decode(&req.data[..])?;

    let name = request.name.unwrap_or_default();
    let guide_id = request.guide_id.unwrap_or(1);
    let step_id = request.step_id.unwrap_or(-1);

    let mut ctx_guard = ctx.lock().await;
    let player_id = ctx_guard.player_id.ok_or(AppError::NotLoggedIn)?;
    let pool = &ctx_guard.state.db;

    rename_user_and_update_guide(pool, player_id, &name, guide_id, step_id)
        .await
        .map_err(AppError::from)?;

    let reply = RenameReply {
        can_rename: Some(true),
        ext_rename: Some(1),
    };

    ctx_guard
        .send_reply(CmdId::RenameCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
