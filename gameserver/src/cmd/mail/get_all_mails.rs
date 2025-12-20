use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::send_reply;
use crate::state::ConnectionContext;
use sonettobuf::{CmdId, GetAllMailsReply};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_all_mails(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    send_reply!(
        ctx,
        req.up_tag,
        CmdId::GetAllMailsCmd,
        GetAllMailsReply,
        "mail/get_all_mails.json"
    );
    Ok(())
}
