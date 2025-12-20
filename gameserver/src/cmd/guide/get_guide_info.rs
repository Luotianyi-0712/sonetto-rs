use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use database::db::game::guides;
use sonettobuf::{CmdId, GetGuideInfoReply};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_guide_info(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let guide_progress = {
        let ctx_guard = ctx.lock().await;
        let player_id = ctx_guard.player_id.ok_or(AppError::NotLoggedIn)?;

        guides::get_all_guide_progress(&ctx_guard.state.db, player_id).await?
    };

    let reply = GetGuideInfoReply {
        guide_infos: guide_progress.into_iter().map(Into::into).collect(),
    };

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::GetGuideInfoCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
