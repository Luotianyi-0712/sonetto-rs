use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use crate::{error::AppError, send_push};
use sonettobuf::{
    CmdId, GainSpecialBlockPush, GetMonthCardInfoReply, MaterialChangePush, MonthCardInfo,
    UpdateRedDotPush,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_month_card_info(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    // Phase 1: check eligibility (read-only lock)
    let (can_claim, current_time) = {
        let ctx_guard = ctx.lock().await;
        let current_time = common::time::ServerTime::now_ms();

        // Check if can claim based on server reset time
        let can_claim = ctx_guard
            .player_state
            .as_ref()
            .map(|s| s.can_claim_month_card(current_time))
            .unwrap_or(false);

        (can_claim, current_time)
    };

    // Phase 1: send rewards
    if can_claim {
        tracing::info!("Claiming month card bonus");

        // these send the birthday blocks bugged for now

        /*  send_push!(
            ctx,
            CmdId::GainSpecialBlockPushCmd,
            GainSpecialBlockPush,
            "charge/gain_special_block_push.json"
        );

        send_push!(
            ctx,
            CmdId::MaterialChangePushCmd,
            MaterialChangePush,
            "charge/material_change_push.json"
        );*/

        send_push!(
            ctx,
            CmdId::UpdateRedDotPushCmd,
            UpdateRedDotPush,
            "charge/update_red_dot_push.json"
        );

        // Phase 3: persist state
        let mut ctx_guard = ctx.lock().await;
        if let Some(state) = ctx_guard.player_state_mut() {
            state.claim_month_card(current_time);
            ctx_guard.save_current_player_state().await?;
        }
    } else {
        tracing::info!("Month card already claimed today");
    }

    // Send reply
    let reply = GetMonthCardInfoReply {
        infos: vec![MonthCardInfo {
            id: Some(610001),
            expire_time: Some(1767607200),
            has_get_bonus: Some(!can_claim),
        }],
    };

    {
        let mut ctx_guard = ctx.lock().await;
        ctx_guard
            .send_reply(CmdId::GetMonthCardInfoCmd, reply, 0, req.up_tag)
            .await?;
    }

    Ok(())
}
