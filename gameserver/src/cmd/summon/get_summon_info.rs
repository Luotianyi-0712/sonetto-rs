use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use database::db::game::summon;
use sonettobuf::{CmdId, GetSummonInfoReply};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_summon_info(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let (stats, pool_infos) = {
        let ctx_guard = ctx.lock().await;
        let player_id = ctx_guard.player_id.ok_or(AppError::NotLoggedIn)?;

        let stats = summon::get_summon_stats(&ctx_guard.state.db, player_id).await?;
        let pools = summon::get_summon_pool_infos(&ctx_guard.state.db, player_id).await?;

        (stats, pools)
    };

    let reply = GetSummonInfoReply {
        free_equip_summon: Some(stats.free_equip_summon),
        is_show_new_summon: Some(stats.is_show_new_summon),
        new_summon_count: Some(stats.new_summon_count),
        pool_infos: pool_infos.into_iter().map(Into::into).collect(),
        total_summon_count: Some(stats.total_summon_count),
    };

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::GetSummonInfoCmd, reply, 0, req.up_tag)
        .await?;
    Ok(())
}
