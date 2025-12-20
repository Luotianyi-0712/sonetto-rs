// src/battle/step_builder.rs

use sonettobuf::{ActEffect, BuffInfo, FightStep, fight_step};

pub struct FightStepBuilder {
    act_type: fight_step::ActType,
    from_id: i64,
    to_id: i64,
    act_id: i32,
    effects: Vec<ActEffect>,
}

#[allow(dead_code)]
impl FightStepBuilder {
    pub fn new_effect() -> Self {
        Self {
            act_type: fight_step::ActType::Effect,
            from_id: 0,
            to_id: 0,
            act_id: 0,
            effects: Vec::new(),
        }
    }

    pub fn new_skill(from_id: i64, to_id: i64, skill_id: i32) -> Self {
        Self {
            act_type: fight_step::ActType::Skill,
            from_id,
            to_id,
            act_id: skill_id,
            effects: Vec::new(),
        }
    }

    pub fn add_nested_step(mut self, step: FightStep) -> Self {
        self.effects.push(ActEffect {
            target_id: Some(0),
            effect_type: Some(162), // FIGHTSTEP wrapper
            fight_step: Some(step),
            ..Default::default()
        });
        self
    }

    pub fn add_buff(mut self, target_id: i64, buff_id: i32, buff_uid: i64, from_uid: i64) -> Self {
        self.effects.push(ActEffect {
            target_id: Some(target_id),
            effect_type: Some(5), // BUFFADD
            effect_num: Some(buff_id),
            buff: Some(BuffInfo {
                buff_id: Some(buff_id),
                duration: Some(0),
                uid: Some(buff_uid),
                ex_info: Some(0),
                from_uid: Some(from_uid),
                count: Some(0),
                layer: Some(1),
                r#type: Some(0),
                act_common_params: Some(String::new()),
                act_info: vec![],
            }),
            ..Default::default()
        });
        self
    }

    pub fn add_indicator_change(mut self, target_id: i64) -> Self {
        self.effects.push(ActEffect {
            target_id: Some(target_id),
            effect_type: Some(26), // INDICATORCHANGE
            effect_num: Some(0),
            ..Default::default()
        });
        self
    }

    pub fn add_card_distribution(
        mut self,
        cards: Vec<sonettobuf::CardInfo>,
        effect_type: i32,
    ) -> Self {
        self.effects.push(ActEffect {
            target_id: Some(0),
            effect_type: Some(effect_type), // 159 or 154
            card_info_list: cards,
            ..Default::default()
        });
        self
    }

    pub fn add_power_generation(mut self, power: i32, team_type: i32) -> Self {
        self.effects.push(ActEffect {
            target_id: Some(0),
            effect_type: Some(310),
            effect_num: Some(power),
            team_type: Some(team_type),
            ..Default::default()
        });
        self
    }

    pub fn add_effect_type(mut self, effect_type: i32) -> Self {
        self.effects.push(ActEffect {
            target_id: Some(0),
            effect_type: Some(effect_type),
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
        });
        self
    }

    pub fn add_effect(mut self, effect: ActEffect) -> Self {
        self.effects.push(effect);
        self
    }

    pub fn build(self) -> FightStep {
        FightStep {
            act_type: Some(self.act_type.into()),
            from_id: Some(self.from_id),
            to_id: Some(self.to_id),
            act_id: Some(self.act_id),
            act_effect: self.effects,
            card_index: Some(0),
            support_hero_id: Some(0),
            fake_timeline: Some(false),
        }
    }
}
