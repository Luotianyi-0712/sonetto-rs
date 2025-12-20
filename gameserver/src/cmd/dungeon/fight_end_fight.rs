use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use prost::Message;
use sonettobuf::{CmdId, EndFightReply, EndFightRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_fight_end_fight(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = EndFightRequest::decode(&req.data[..])?;

    let is_abort = request.is_abort.ok_or(AppError::InvalidRequest)?;

    tracing::info!("Fight ended with is_abort: {}", is_abort);

    // Clear battle
    {
        let mut ctx_guard = ctx.lock().await;
        ctx_guard.active_battle = None;
    }

    let data = EndFightReply {};

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::FightEndFightCmd, data, 0, req.up_tag)
        .await?;
    Ok(())
}
