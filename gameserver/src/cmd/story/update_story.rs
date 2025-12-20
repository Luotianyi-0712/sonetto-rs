use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use prost::Message;
use sonettobuf::{CmdId, UpdateStoryReply, UpdateStoryRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_update_story(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = UpdateStoryRequest::decode(&req.data[..])?;

    tracing::info!("Received update story request: {:?}", request);

    let reply = UpdateStoryReply {};

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::UpdateStoryCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
