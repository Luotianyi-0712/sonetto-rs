use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::send_reply;
use crate::state::ConnectionContext;
use sonettobuf::{CmdId, GetAct208InfoReply};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_act208_info(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    send_reply!(
        ctx,
        req.up_tag,
        CmdId::GetAct208InfoCmd,
        GetAct208InfoReply,
        "activity208/get_info.json"
    );
    Ok(())
}
