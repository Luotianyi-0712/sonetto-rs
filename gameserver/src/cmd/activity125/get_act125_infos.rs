use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use crate::{data_loader::GameDataLoader, error::AppError};
use prost::Message;
use sonettobuf::{CmdId, GetAct125InfosReply, GetAct125InfosRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_act125_infos(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = GetAct125InfosRequest::decode(&req.data[..])?;
    let activity_id = request.activity_id.unwrap_or(0);

    tracing::info!("Requested activity_id: {}", activity_id);

    let path = match activity_id {
        13116 => "activity125/activity125_infos_13116.json",
        13005 => "activity125/activity125_infos_13005.json",
        _ => {
            tracing::warn!("Unknown activity_id: {}, using default", activity_id);
            "activity125/activity125_infos_13116.json"
        }
    };

    let reply: GetAct125InfosReply = GameDataLoader::load_struct(path)
        .map_err(|e| AppError::Custom(format!("Failed to load: {}", e)))?;

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::GetAct125InfosCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
