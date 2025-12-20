use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use common::time::ServerTime;
use sonettobuf::{CmdId, GetServerTimeReply};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_server_time(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetServerTimeReply {
        server_time: Some(ServerTime::now_ms()),
        offset_time: Some(-18000000),
    };

    {
        let mut ctx_guard = ctx.lock().await;
        ctx_guard
            .send_reply_with_down_tag(CmdId::GetServerTimeCmd, data, 0, req.up_tag, 255)
            .await?;
    }

    Ok(())
}
