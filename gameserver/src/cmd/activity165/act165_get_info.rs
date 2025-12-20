use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::send_reply;
use crate::state::ConnectionContext;
use sonettobuf::{Act165GetInfoReply, CmdId};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_act165_get_info(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    send_reply!(
        ctx,
        req.up_tag,
        CmdId::Act165GetInfoCmd,
        Act165GetInfoReply,
        "activity165/get_info.json"
    );
    Ok(())
}
