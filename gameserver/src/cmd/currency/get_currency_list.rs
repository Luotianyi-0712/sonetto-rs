use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use database::db::game::currencies;
use prost::Message;
use sonettobuf::{CmdId, GetCurrencyListReply, GetCurrencyListRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_currency_list(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let req_msg = GetCurrencyListRequest::decode(&req.data[..])?;

    tracing::info!("Requested currency_ids: {:?}", req_msg.currency_ids);

    let currency_list = {
        let ctx_guard = ctx.lock().await;
        let player_id = ctx_guard.player_id.ok_or(AppError::NotLoggedIn)?;

        currencies::get_currencies(&ctx_guard.state.db, player_id, &req_msg.currency_ids).await?
    };

    let reply = GetCurrencyListReply {
        currency_list: currency_list.into_iter().map(Into::into).collect(),
    };

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::GetCurrencyListCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
