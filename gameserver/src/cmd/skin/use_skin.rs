use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use database::db::game::heroes;
use prost::Message;
use sonettobuf::{CmdId, HeroUpdatePush, UseSkinReply, UseSkinRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_use_skin(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = UseSkinRequest::decode(&req.data[..])?;
    tracing::info!("Received UseSkinRequest: {:?}", request);

    let hero_id = request.hero_id.ok_or(AppError::InvalidRequest)?;
    let skin_id = request.skin_id.ok_or(AppError::InvalidRequest)?;

    let updated_hero = {
        let ctx_guard = ctx.lock().await;
        let player_id = ctx_guard.player_id.ok_or(AppError::NotLoggedIn)?;
        let pool = &ctx_guard.state.db;

        // Get hero
        let mut hero = heroes::get_hero_by_hero_id(pool, player_id, hero_id).await?;

        // Update skin
        hero.update_skin(pool, skin_id).await?;

        tracing::info!(
            "User {} equipped skin {} on hero {}",
            player_id,
            skin_id,
            hero_id
        );
        hero
    };

    let data = UseSkinReply {
        hero_id: Some(hero_id),
        skin_id: Some(skin_id),
    };

    {
        let mut ctx_guard = ctx.lock().await;

        let hero_proto: sonettobuf::HeroInfo = updated_hero.into();
        let push = HeroUpdatePush {
            hero_updates: vec![hero_proto],
        };

        ctx_guard
            .send_push(CmdId::HeroHeroUpdatePushCmd, push)
            .await?;

        ctx_guard
            .send_reply(CmdId::UseSkinCmd, data, 0, req.up_tag)
            .await?;
    }

    Ok(())
}
