use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use database::db::game::dungeons::load_dungeon_record;
use prost::Message;
use sonettobuf::{CmdId, GetFightRecordGroupReply, GetFightRecordGroupRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_fight_record_group(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = GetFightRecordGroupRequest::decode(&req.data[..])?;
    let episode_id = request.episode_id.unwrap_or(0);

    tracing::info!("GetFightRecordGroup for episode {}", episode_id);

    let (pool, player_id) = {
        let ctx_guard = ctx.lock().await;
        (ctx_guard.state.db.clone(), ctx_guard.player_id)
    };

    // Try to load record from database
    let record = load_dungeon_record(&pool, player_id.unwrap_or(0), episode_id).await?;

    let reply = GetFightRecordGroupReply {
        fight_group: record,
    };

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::GetFightRecordGroupCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
