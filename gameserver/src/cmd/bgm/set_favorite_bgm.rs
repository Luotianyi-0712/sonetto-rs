use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use database::db::game::bgm::set_bgm_favorite;
use prost::Message;
use sonettobuf::{CmdId, SetFavoriteBgmReply, SetFavoriteBgmRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_set_favorite_bgm(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = SetFavoriteBgmRequest::decode(&req.data[..])?;

    let mut ctx_guard = ctx.lock().await;
    let player_id = ctx_guard.player_id.ok_or(AppError::NotLoggedIn)?;
    let pool = &ctx_guard.state.db;

    set_bgm_favorite(
        pool,
        player_id,
        request.bgm_id.unwrap_or(2207),
        request.favorite.unwrap_or(false),
    )
    .await
    .map_err(AppError::from)?;

    let reply = SetFavoriteBgmReply {
        bgm_id: request.bgm_id,
        favorite: request.favorite,
    };

    ctx_guard
        .send_reply(CmdId::SetFavoriteBgmCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
