use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use prost::Message;
use sonettobuf::{CmdId, TalentStyleReadReply, TalentStyleReadRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_talent_style_read(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = TalentStyleReadRequest::decode(&req.data[..])?;

    let data = TalentStyleReadReply {
        hero_id: Some(request.hero_id.unwrap_or(3080)),
    };

    {
        let mut ctx_guard = ctx.lock().await;
        ctx_guard
            .send_reply(CmdId::TalentStyleReadCmd, data, 0, req.up_tag)
            .await?;
    }

    Ok(())
}
