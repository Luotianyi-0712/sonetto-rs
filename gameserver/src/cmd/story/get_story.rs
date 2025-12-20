use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use database::db::game::stories;
use sonettobuf::{CmdId, GetStoryReply};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_story(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let (finished, processing) = {
        let ctx_guard = ctx.lock().await;
        let player_id = ctx_guard.player_id.ok_or(AppError::NotLoggedIn)?;

        let finished = stories::get_finished_stories(&ctx_guard.state.db, player_id).await?;
        let processing = stories::get_processing_stories(&ctx_guard.state.db, player_id).await?;

        (finished, processing)
    };

    let reply = GetStoryReply {
        finish_list: finished,
        processing_list: processing.into_iter().map(Into::into).collect(),
    };

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::GetStoryCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
