use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use database::db::game::activity101;
use prost::Message;
use sonettobuf::{Act101Info, CmdId, Get101InfosReply, Get101InfosRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get101_infos(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = Get101InfosRequest::decode(&req.data[..])?;
    let activity_id = request.activity_id.unwrap_or(13108);

    tracing::info!("Requested activity_id: {}", activity_id);

    let (infos, login_count, got_once_bonus) = {
        let ctx_guard = ctx.lock().await;
        let player_id = ctx_guard.player_id.ok_or(AppError::NotLoggedIn)?;

        activity101::get_activity101_info(&ctx_guard.state.db, player_id, activity_id).await?
    };

    let reply = Get101InfosReply {
        infos: infos
            .into_iter()
            .map(|(id, state)| Act101Info {
                id: Some(id as u32),
                state: Some(state as u32),
            })
            .collect(),
        sp_infos: vec![], // Special infos if needed
        login_count: Some(login_count as u32),
        activity_id: Some(activity_id),
        got_once_bonus: Some(got_once_bonus),
    };

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::Get101InfosCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
