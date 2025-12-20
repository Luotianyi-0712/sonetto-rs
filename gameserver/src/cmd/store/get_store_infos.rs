use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use crate::{data_loader::GameDataLoader, error::AppError};
use prost::Message;
use sonettobuf::{CmdId, GetStoreInfosReply, GetStoreInfosRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_store_infos(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = GetStoreInfosRequest::decode(&req.data[..])?;
    let store_ids = request.store_ids;
    tracing::info!("Requested store_ids: {:?}", store_ids);

    // Load master store file
    let response: GetStoreInfosReply = GameDataLoader::load_struct("store/store_infos.json")
        .map_err(|e| AppError::Custom(format!("Failed to load store infos: {}", e)))?;

    let reply = if store_ids.is_empty() {
        // Return all stores
        tracing::info!("No specific store IDs requested, returning all stores");
        response
    } else {
        // Filter to only requested store IDs (with all their goods)
        GetStoreInfosReply {
            store_infos: response
                .store_infos
                .into_iter()
                .filter(|store| store_ids.contains(&store.id))
                .collect(),
        }
    };

    tracing::info!("Returning {} store(s)", reply.store_infos.len());

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::GetStoreInfosCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
