use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use sonettobuf::{CmdId, GetHeroGroupCommonListReply};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_hero_group_common_list(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let (common_groups, type_groups) = {
        let ctx_guard = ctx.lock().await;
        let player_id = ctx_guard.player_id.ok_or(AppError::NotLoggedIn)?;

        let commons =
            database::db::game::hero_groups::get_hero_groups_common(&ctx_guard.state.db, player_id)
                .await?;
        let types =
            database::db::game::hero_groups::get_hero_group_types(&ctx_guard.state.db, player_id)
                .await?;

        (commons, types)
    };

    let reply = GetHeroGroupCommonListReply {
        hero_group_commons: common_groups.into_iter().map(Into::into).collect(),
        hero_gourp_types: type_groups.into_iter().map(Into::into).collect(),
    };

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::GetHeroGroupCommonListCmd, reply, 0, req.up_tag)
        .await?;
    Ok(())
}
