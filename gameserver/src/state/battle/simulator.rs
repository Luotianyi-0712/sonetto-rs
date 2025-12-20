// src/battle/simulator.rs

use anyhow::Result;
use rand::SeedableRng;
use rand::rngs::StdRng;
use sonettobuf::{
    ActEffect, BeginRoundOper, CardInfo, Fight, FightEntityInfo, FightRound, FightStep, fight_step,
};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct BattleSimulator {
    fight: Fight,
    rng: StdRng,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl BattleSimulator {
    pub fn new(fight: Fight) -> Self {
        let seed = chrono::Utc::now().timestamp_millis() as u64;
        Self {
            fight,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub async fn process_round(
        &mut self,
        operations: Vec<BeginRoundOper>,
        current_deck: Vec<CardInfo>,
    ) -> Result<FightRound> {
        let mut state = RoundState::from_fight(&self.fight)?;
        state.player_deck = current_deck;

        let mut steps = Vec::new();

        // Process player operations
        for oper in operations {
            let step = self.execute_operation(&mut state, oper).await?;
            if step.act_type.is_some() && step.act_type.unwrap() != 0 {
                steps.push(step);
            }
        }

        // Add round end effect
        steps.push(FightStep {
            act_type: Some(fight_step::ActType::Effect.into()),
            from_id: Some(0),
            to_id: Some(0),
            act_id: Some(0),
            act_effect: vec![ActEffect {
                target_id: Some(0),
                effect_type: Some(61), // ROUNDEND
                effect_num: Some(0),
                buff: None,
                entity: None,
                config_effect: Some(0),
                buff_act_id: Some(0),
                reserve_id: Some(0),
                reserve_str: Some(String::new()),
                summoned: None,
                magic_circle: None,
                card_info: None,
                card_info_list: vec![],
                team_type: Some(0),
                fight_step: None,
                assist_boss_info: None,
                effect_num1: Some(0),
                emitter_info: None,
                player_finisher_info: None,
                power_info: None,
                card_heat_value: None,
                fight_tasks: vec![],
                fight: None,
                buff_act_info: None,
                hurt_info: None,
            }],
            card_index: Some(0),
            support_hero_id: Some(0),
            fake_timeline: Some(false),
        });

        // Increment move counter
        state.move_num += 1;

        Ok(self.build_round_response(steps, state))
    }

    async fn execute_operation(
        &mut self,
        state: &mut RoundState,
        oper: BeginRoundOper,
    ) -> Result<FightStep> {
        let oper_type = oper.oper_type.unwrap_or(0);

        match oper_type {
            1 => self.play_card(state, oper).await,
            2 => {
                // Check if this is card selection or actual player skill
                let param1 = oper.param1.unwrap_or(0);
                if param1 < 100 {
                    // Low param1 = card selection (0, 1, 2, etc)
                    self.select_card(state, oper).await
                } else {
                    // High param1 = cloth skill ID
                    self.use_player_skill(state, oper).await
                }
            }
            3 => self.change_hero(state, oper).await,
            4 => self.end_turn(state).await,
            _ => Ok(FightStep::default()),
        }
    }

    async fn select_card(
        &mut self,
        state: &mut RoundState,
        oper: BeginRoundOper,
    ) -> Result<FightStep> {
        let card_index = oper.param1.unwrap_or(0);

        tracing::info!("Selecting card at index {}", card_index);

        // Return a step indicating card was selected
        Ok(FightStep {
            act_type: Some(fight_step::ActType::Effect.into()),
            from_id: Some(0),
            to_id: Some(0),
            act_id: Some(0),
            act_effect: vec![ActEffect {
                target_id: Some(0),
                effect_type: Some(149), // ADDHANDCARD
                effect_num: Some(card_index),
                buff: None,
                entity: None,
                config_effect: Some(0),
                buff_act_id: Some(0),
                reserve_id: Some(0),
                reserve_str: Some(String::new()),
                summoned: None,
                magic_circle: None,
                card_info: None,
                card_info_list: vec![],
                team_type: Some(1),
                fight_step: None,
                assist_boss_info: None,
                effect_num1: Some(0),
                emitter_info: None,
                player_finisher_info: None,
                power_info: None,
                card_heat_value: None,
                fight_tasks: vec![],
                fight: None,
                buff_act_info: None,
                hurt_info: None,
            }],
            card_index: Some(card_index),
            support_hero_id: Some(0),
            fake_timeline: Some(false),
        })
    }

    async fn play_card(
        &mut self,
        state: &mut RoundState,
        oper: BeginRoundOper,
    ) -> Result<FightStep> {
        let card_index = oper.param1.unwrap_or(0) as usize;
        let target_uid = oper.to_id.unwrap_or(0);

        // TODO: Get card from player deck
        // TODO: Load skill config from excel
        // TODO: Execute skill effects
        // TODO: Update state (HP, buffs, etc)
        // TODO: Consume action points

        tracing::info!("Playing card {} targeting {}", card_index, target_uid);

        // Placeholder: return empty step for now
        Ok(FightStep {
            act_type: Some(fight_step::ActType::Skill.into()),
            from_id: Some(0),
            to_id: Some(target_uid),
            act_id: Some(0),
            act_effect: vec![],
            card_index: Some(card_index as i32),
            support_hero_id: Some(0),
            fake_timeline: Some(false),
        })
    }

    async fn use_player_skill(
        &mut self,
        state: &mut RoundState,
        oper: BeginRoundOper,
    ) -> Result<FightStep> {
        tracing::info!("Using player skill");
        Ok(FightStep::default())
    }

    async fn change_hero(
        &mut self,
        state: &mut RoundState,
        oper: BeginRoundOper,
    ) -> Result<FightStep> {
        tracing::info!("Changing hero");
        Ok(FightStep::default())
    }

    async fn end_turn(&mut self, state: &mut RoundState) -> Result<FightStep> {
        tracing::info!("Ending turn");
        Ok(FightStep::default())
    }

    async fn execute_ai_turn(&mut self, state: &mut RoundState) -> Result<Vec<FightStep>> {
        // TODO: AI logic
        Ok(vec![])
    }

    async fn process_round_end(&mut self, state: &mut RoundState) -> Result<Vec<FightStep>> {
        // TODO: End of round processing
        Ok(vec![])
    }
    fn check_battle_end(&self, state: &RoundState) -> bool {
        // TODO: Check if all enemies dead (win)
        // TODO: Check if all heroes dead (lose)
        false
    }

    fn build_round_response(&self, steps: Vec<FightStep>, state: RoundState) -> FightRound {
        // Build ex_point_info from current entity states
        let ex_point_info = state.build_ex_point_info();

        FightRound {
            fight_step: steps,
            act_point: Some(state.act_point),
            is_finish: Some(state.is_finish),
            move_num: Some(state.move_num + 1), // Increment move counter
            ex_point_info,
            ai_use_cards: state.ai_cards,
            power: Some(state.power),
            skill_infos: vec![],
            before_cards1: vec![],
            team_a_cards1: state.player_deck, // Updated deck
            before_cards2: vec![],
            team_a_cards2: vec![],
            next_round_begin_step: vec![],
            use_card_list: state.used_cards,
            cur_round: Some(state.round_num),
            hero_sp_attributes: vec![],
            last_change_hero_uid: Some(0),
        }
    }
}

struct RoundState {
    act_point: i32,
    power: i32,
    player_deck: Vec<CardInfo>,
    ai_cards: Vec<CardInfo>,
    entities: HashMap<i64, FightEntityInfo>,
    used_cards: Vec<i32>,
    round_num: i32,
    move_num: i32,
    is_finish: bool,
}

#[allow(dead_code)]
impl RoundState {
    fn from_fight(fight: &Fight) -> Result<Self> {
        let mut entities = HashMap::new();

        // Load attacker entities
        if let Some(ref attacker) = fight.attacker {
            for entity in &attacker.entitys {
                if let Some(uid) = entity.uid {
                    entities.insert(uid, entity.clone());
                }
            }
        }

        // Load defender entities
        if let Some(ref defender) = fight.defender {
            for entity in &defender.entitys {
                if let Some(uid) = entity.uid {
                    entities.insert(uid, entity.clone());
                }
            }
        }

        Ok(Self {
            act_point: 4,
            power: 15,
            player_deck: vec![], // Will be set from active_battle
            ai_cards: vec![],
            entities,
            used_cards: vec![],
            round_num: fight.cur_round.unwrap_or(1),
            move_num: 0,
            is_finish: false,
        })
    }

    fn get_entity(&self, uid: i64) -> Option<&FightEntityInfo> {
        self.entities.get(&uid)
    }

    fn get_entity_mut(&mut self, uid: i64) -> Option<&mut FightEntityInfo> {
        self.entities.get_mut(&uid)
    }

    fn build_ex_point_info(&self) -> Vec<sonettobuf::FightExPointInfo> {
        self.entities
            .values()
            .map(|entity| sonettobuf::FightExPointInfo {
                uid: entity.uid,
                ex_point: entity.ex_point,
                power_infos: entity.power_infos.clone(),
                current_hp: entity.current_hp,
                ex_point_type: entity.ex_point_type,
            })
            .collect()
    }
}
