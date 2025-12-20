use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use database::db::game::friends;
use sonettobuf::{CmdId, LoadFriendInfosReply};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_load_friend_infos(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let (friend_ids, blacklist_ids) = {
        let ctx_guard = ctx.lock().await;
        let player_id = ctx_guard.player_id.ok_or(AppError::NotLoggedIn)?;

        let friends = friends::get_friend_ids(&ctx_guard.state.db, player_id).await?;
        let blacklist = friends::get_blacklist_ids(&ctx_guard.state.db, player_id).await?;

        (friends, blacklist)
    };

    let reply = LoadFriendInfosReply {
        friend_ids,
        black_list_ids: blacklist_ids,
    };

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::LoadFriendInfosCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
