// src/battle/round_builder.rs

use anyhow::Result;
use sonettobuf::{Fight, FightExPointInfo, FightRound};

use crate::state::battle::step_builder::FightStepBuilder;

pub async fn build_initial_round(
    fight: &Fight,
    card_deck: Vec<sonettobuf::CardInfo>,
) -> Result<FightRound> {
    let mut steps = vec![];

    // Add ENTERFIGHTDEAL with all fields initialized
    steps.push(
        FightStepBuilder::new_effect()
            .add_effect_type(233) // ENTERFIGHTDEAL
            .build(),
    );

    // Power generation (already working)
    steps.push(
        FightStepBuilder::new_effect()
            .add_power_generation(15, 1)
            .build(),
    );

    let ex_point_info = build_ex_point_info(fight);

    Ok(FightRound {
        fight_step: steps,
        act_point: Some(4),
        is_finish: Some(false),
        move_num: Some(0),
        ex_point_info,
        ai_use_cards: vec![],
        power: Some(15),
        skill_infos: vec![],
        before_cards1: vec![],
        team_a_cards1: card_deck,
        before_cards2: vec![],
        team_a_cards2: vec![],
        next_round_begin_step: vec![],
        use_card_list: vec![],
        cur_round: Some(1),
        hero_sp_attributes: vec![],
        last_change_hero_uid: Some(0),
    })
}

fn build_ex_point_info(fight: &Fight) -> Vec<FightExPointInfo> {
    let mut ex_point_info = Vec::new();

    if let Some(ref attacker) = fight.attacker {
        for entity in &attacker.entitys {
            ex_point_info.push(FightExPointInfo {
                uid: entity.uid,
                ex_point: entity.ex_point,
                power_infos: entity.power_infos.clone(),
                current_hp: entity.current_hp,
                ex_point_type: entity.ex_point_type,
            });
        }
    }

    if let Some(ref defender) = fight.defender {
        for entity in &defender.entitys {
            ex_point_info.push(FightExPointInfo {
                uid: entity.uid,
                ex_point: entity.ex_point,
                power_infos: entity.power_infos.clone(),
                current_hp: entity.current_hp,
                ex_point_type: entity.ex_point_type,
            });
        }
    }

    ex_point_info
}
