use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use database::db::game::charges;
use sonettobuf::{CmdId, GetChargeInfoReply};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_charge_info(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let (charge_infos, sandbox) = {
        let ctx_guard = ctx.lock().await;
        let player_id = ctx_guard.player_id.ok_or(AppError::NotLoggedIn)?;

        let infos = charges::get_charge_infos(&ctx_guard.state.db, player_id).await?;
        let sandbox = charges::get_sandbox_settings(&ctx_guard.state.db, player_id).await?;

        (infos, sandbox)
    };

    let reply = GetChargeInfoReply {
        infos: charge_infos.into_iter().map(Into::into).collect(),
        sandbox_enable: Some(sandbox.sandbox_enable),
        sandbox_balance: Some(sandbox.sandbox_balance),
    };

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::GetChargeInfoCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
