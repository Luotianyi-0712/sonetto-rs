use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::send_reply;
use crate::state::ConnectionContext;
use sonettobuf::{CmdId, GetActivityInfosReply};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_activity_infos(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    send_reply!(
        ctx,
        req.up_tag,
        CmdId::GetActivityInfosCmd,
        GetActivityInfosReply,
        "activity/activity_infos.json"
    );
    Ok(())
}
