// src/battle/entity_builder.rs

use data::exceldb;
use database::db::game::{equipment, heroes::HeroData};
use sonettobuf::{EquipRecord, FightEntityInfo, HeroAttribute};
use sqlx::SqlitePool;

pub async fn build_hero_entity(
    pool: &SqlitePool,
    hero_data: &HeroData,
    position: i32,
    team_type: i32,
) -> FightEntityInfo {
    let record = &hero_data.record;

    let equip_id = equipment::get_hero_default_equip_id(pool, record.uid, record.user_id)
        .await
        .ok()
        .flatten();

    FightEntityInfo {
        uid: Some(record.uid),
        model_id: Some(record.hero_id),
        skin: Some(record.skin),
        position: Some(position),
        entity_type: Some(1), // 1 = Hero
        user_id: Some(record.user_id),
        ex_point: Some(0),
        level: Some(record.level),
        current_hp: Some(record.base_hp),
        attr: Some(HeroAttribute {
            hp: Some(record.base_hp),
            attack: Some(record.base_attack),
            defense: Some(record.base_defense),
            mdefense: Some(record.base_mdefense),
            technic: Some(record.base_technic),
            multi_hp_idx: Some(record.base_multi_hp_idx),
            multi_hp_num: Some(record.base_multi_hp_num),
        }),
        buffs: vec![], // Filled in round_builder
        skill_group1: get_hero_skill_group(record.hero_id, 1),
        skill_group2: get_hero_skill_group(record.hero_id, 2),
        passive_skill: get_hero_passive_skills(&hero_data, equip_id),
        ex_skill: Some(get_hero_ex_skill(&hero_data)),
        shield_value: Some(0),
        no_effect_buffs: vec![],
        expoint_max_add: Some(0),
        buff_harm_statistic: Some(0),
        equip_uid: Some(record.default_equip_uid), // TODO: Load from hero equipment
        trial_equip: Some(EquipRecord {
            equip_uid: Some(0),
            equip_id: Some(0),
            equip_lv: Some(0),
            refine_lv: Some(0),
        }),
        ex_skill_level: Some(record.ex_skill_level),
        power_infos: vec![],
        act104_equip_uids: vec![],
        trial_act104_equips: vec![],
        summoned_list: vec![],
        base_attr: Some(HeroAttribute {
            hp: Some(record.base_hp),
            attack: Some(record.base_attack),
            defense: Some(record.base_defense),
            mdefense: Some(record.base_mdefense),
            technic: Some(record.base_technic),
            multi_hp_idx: Some(record.base_multi_hp_idx),
            multi_hp_num: Some(record.base_multi_hp_num),
        }),
        ex_skill_point_change: Some(0),
        team_type: Some(team_type),
        enhance_info_box: Some(sonettobuf::EnhanceInfoBox {
            uid: Some(record.uid),
            can_upgrade_ids: vec![],
            upgraded_options: vec![],
        }),
        trial_id: Some(0),
        career: Some(get_hero_career(&hero_data)),
        status: Some(0),
        guard: Some(-1),
        sub_cd: Some(0),
        ex_point_type: Some(0),
        equips: vec![], // game has this empty???
        destiny_stone: Some(record.destiny_stone),
        destiny_rank: Some(record.destiny_rank),
        custom_unit_id: Some(0),
    }
}

pub fn build_player_entity(user_id: i64, team_type: i32) -> FightEntityInfo {
    let uid = if team_type == 1 { 0 } else { -99999 };

    FightEntityInfo {
        uid: Some(uid),
        model_id: Some(0),
        skin: Some(0),
        position: Some(0),
        entity_type: Some(3), // 3 = Player entity
        user_id: Some(user_id),
        ex_point: Some(0),
        level: Some(0),
        current_hp: Some(100),
        attr: Some(HeroAttribute {
            hp: Some(100),
            attack: Some(0),
            defense: Some(0),
            mdefense: Some(0),
            technic: Some(0),
            multi_hp_idx: Some(0),
            multi_hp_num: Some(0),
        }),
        buffs: vec![],
        skill_group1: vec![],
        skill_group2: vec![],
        passive_skill: vec![],
        ex_skill: Some(0),
        shield_value: Some(0),
        no_effect_buffs: vec![],
        expoint_max_add: Some(0),
        buff_harm_statistic: Some(0),
        equip_uid: Some(0),
        trial_equip: Some(EquipRecord {
            equip_uid: Some(0),
            equip_id: Some(0),
            equip_lv: Some(0),
            refine_lv: Some(0),
        }),
        ex_skill_level: Some(0),
        power_infos: vec![],
        act104_equip_uids: vec![],
        trial_act104_equips: vec![],
        summoned_list: vec![],
        base_attr: Some(HeroAttribute {
            hp: Some(100),
            attack: Some(0),
            defense: Some(0),
            mdefense: Some(0),
            technic: Some(0),
            multi_hp_idx: Some(0),
            multi_hp_num: Some(0),
        }),
        ex_skill_point_change: Some(0),
        team_type: Some(team_type),
        enhance_info_box: Some(sonettobuf::EnhanceInfoBox {
            uid: Some(uid),
            can_upgrade_ids: vec![],
            upgraded_options: vec![],
        }),
        trial_id: Some(0),
        career: Some(0),
        status: Some(0),
        guard: Some(-1),
        sub_cd: Some(0),
        ex_point_type: Some(0),
        equips: vec![],
        destiny_stone: Some(0),
        destiny_rank: Some(0),
        custom_unit_id: Some(0),
    }
}

pub fn get_hero_skill_group(hero_id: i32, group: i32) -> Vec<i32> {
    let game_data = exceldb::get();

    let character = game_data.character.iter().find(|c| c.id == hero_id);

    if let Some(character) = character {
        // Parse skill string: "1#30860111#30860112#30860113|2#30860121#30860122#30860123"
        for group_str in character.skill.split('|') {
            let parts: Vec<&str> = group_str.split('#').collect();

            // First part is group number, check if it matches
            if let Some(first) = parts.first() {
                if let Ok(group_num) = first.parse::<i32>() {
                    if group_num == group {
                        // Return the skill IDs (skip first element which is the group number)
                        return parts[1..]
                            .iter()
                            .filter_map(|s| s.parse::<i32>().ok())
                            .collect();
                    }
                }
            }
        }
    }

    vec![]
}

// Make it non-async, pass equip_id from outside
fn get_hero_passive_skills(hero_data: &HeroData, equip_id: Option<i32>) -> Vec<i32> {
    let game_data = exceldb::get();
    let mut passives = Vec::new();
    let hero_id = hero_data.record.hero_id;

    // Get ex passives
    let mut ex_passives = Vec::new();
    if let Some(ex_level) = game_data
        .skill_ex_level
        .iter()
        .find(|s| s.hero_id == hero_id && s.skill_level == hero_data.record.ex_skill_level)
    {
        for group in ex_level.passive_skill.split('|') {
            if let Some((_, passive)) = group.split_once('#') {
                if let Ok(id) = passive.parse::<i32>() {
                    ex_passives.push(id);
                }
            }
        }
    }

    if let Some(p) = ex_passives.get(0) {
        passives.push(*p);
    }

    // Passive level skill
    if let Some(passive_level) = game_data
        .skill_passive_level
        .iter()
        .find(|s| s.hero_id == hero_id && s.skill_level == 2)
    {
        passives.push(passive_level.skill_passive);
    }

    if let Some(p) = ex_passives.get(1) {
        passives.push(*p);
    }

    // Equipment passive skills
    if let Some(equip_id) = equip_id {
        if let Some(equip_skill) = game_data.equip_skill.iter().find(|e| e.id == equip_id) {
            if equip_skill.skill != 0 {
                passives.push(equip_skill.skill);
            }
            if equip_skill.skill2 != 0 {
                passives.push(equip_skill.skill2);
            }
        }
    }

    passives
}

fn get_hero_ex_skill(hero_data: &HeroData) -> i32 {
    let game_data = exceldb::get();
    let hero_id = hero_data.record.hero_id;
    let ex_skill_level = hero_data.record.ex_skill_level;

    // Get the skill_ex for this hero at their current ex_skill_level
    game_data
        .skill_ex_level
        .iter()
        .find(|s| s.hero_id == hero_id && s.skill_level == ex_skill_level)
        .map(|s| s.skill_ex)
        .unwrap_or(0)
}

fn get_hero_career(hero_data: &HeroData) -> i32 {
    let game_data = exceldb::get();
    let hero_id = hero_data.record.hero_id;

    // Get the career for this hero
    game_data
        .character
        .iter()
        .find(|s| s.id == hero_id)
        .map(|s| s.career)
        .unwrap_or(0)
}
