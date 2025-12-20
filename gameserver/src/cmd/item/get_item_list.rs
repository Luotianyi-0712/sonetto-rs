use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use database::db::game::items;
use sonettobuf::{CmdId, GetItemListReply};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_item_list(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let (items_data, power_items_data, insight_items_data) = {
        let ctx_guard = ctx.lock().await;
        let user_id = ctx_guard.player_id.ok_or(AppError::NotLoggedIn)?;

        let items = items::get_all_items(&ctx_guard.state.db, user_id).await?;
        let power_items = items::get_all_power_items(&ctx_guard.state.db, user_id).await?;
        let insight_items = items::get_all_insight_items(&ctx_guard.state.db, user_id).await?;

        (items, power_items, insight_items)
    };

    let reply = GetItemListReply {
        items: items_data.into_iter().map(Into::into).collect(),
        power_items: power_items_data.into_iter().map(Into::into).collect(),
        insight_items: insight_items_data.into_iter().map(Into::into).collect(),
    };

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::GetItemListCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
