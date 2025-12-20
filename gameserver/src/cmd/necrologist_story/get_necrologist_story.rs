use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::send_reply;
use crate::state::ConnectionContext;
use sonettobuf::{CmdId, GetNecrologistStoryReply};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_necrologist_story(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    send_reply!(
        ctx,
        req.up_tag,
        CmdId::GetNecrologistStoryCmd,
        GetNecrologistStoryReply,
        "necrologist_story/necrologist_story.json"
    );
    Ok(())
}
