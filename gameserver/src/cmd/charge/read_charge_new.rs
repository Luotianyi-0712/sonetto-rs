use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use prost::Message;
use sonettobuf::{CmdId, ReadChargeNewReply, ReadChargeNewRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_read_charge_new(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = ReadChargeNewRequest::decode(&req.data[..])?;

    tracing::info!("Received ReadChargeNewRequest: {:?}", request);

    let data = ReadChargeNewReply {
        goods_ids: request.goods_ids,
    };

    {
        let mut ctx_guard = ctx.lock().await;
        ctx_guard
            .send_reply(CmdId::ReadChargeNewCmd, data, 0, req.up_tag)
            .await?;
    }

    Ok(())
}
