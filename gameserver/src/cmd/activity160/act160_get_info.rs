use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::send_reply;
use crate::state::ConnectionContext;
use sonettobuf::{Act160GetInfoReply, CmdId};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_act160_get_info(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    send_reply!(
        ctx,
        req.up_tag,
        CmdId::Act160GetInfoCmd,
        Act160GetInfoReply,
        "activity160/get_info.json"
    );
    Ok(())
}
