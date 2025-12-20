use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use database::db::game::bgm::load_user_bgm;
use sonettobuf::{CmdId, GetBgmInfoReply};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_bgm_info(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let (bgm_infos, use_bgm_id) = {
        let ctx_guard = ctx.lock().await;
        let player_id = ctx_guard.player_id.ok_or(AppError::NotLoggedIn)?;

        load_user_bgm(&ctx_guard.state.db, player_id).await?
    };

    let reply = GetBgmInfoReply {
        bgm_infos,
        use_bgm_id,
    };

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::GetBgmInfoCmd, reply, 0, req.up_tag)
        .await?;
    Ok(())
}
