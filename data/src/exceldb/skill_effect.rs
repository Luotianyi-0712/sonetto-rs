// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillEffect {
    #[serde(rename = "behavior1")]
    pub behavior1: String,
    #[serde(rename = "behavior10")]
    pub behavior10: String,
    #[serde(rename = "behavior11")]
    pub behavior11: String,
    #[serde(rename = "behavior12")]
    pub behavior12: String,
    #[serde(rename = "behavior13")]
    pub behavior13: String,
    #[serde(rename = "behavior14")]
    pub behavior14: String,
    #[serde(rename = "behavior15")]
    pub behavior15: String,
    #[serde(rename = "behavior16")]
    pub behavior16: String,
    #[serde(rename = "behavior17")]
    pub behavior17: String,
    #[serde(rename = "behavior18")]
    pub behavior18: String,
    #[serde(rename = "behavior19")]
    pub behavior19: String,
    #[serde(rename = "behavior2")]
    pub behavior2: String,
    #[serde(rename = "behavior20")]
    pub behavior20: String,
    #[serde(rename = "behavior3")]
    pub behavior3: String,
    #[serde(rename = "behavior4")]
    pub behavior4: String,
    #[serde(rename = "behavior5")]
    pub behavior5: String,
    #[serde(rename = "behavior6")]
    pub behavior6: String,
    #[serde(rename = "behavior7")]
    pub behavior7: String,
    #[serde(rename = "behavior8")]
    pub behavior8: String,
    #[serde(rename = "behavior9")]
    pub behavior9: String,
    #[serde(rename = "behaviorTarget1")]
    pub behavior_target1: i32,
    #[serde(rename = "behaviorTarget10")]
    pub behavior_target10: i32,
    #[serde(rename = "behaviorTarget11")]
    pub behavior_target11: i32,
    #[serde(rename = "behaviorTarget12")]
    pub behavior_target12: i32,
    #[serde(rename = "behaviorTarget13")]
    pub behavior_target13: i32,
    #[serde(rename = "behaviorTarget14")]
    pub behavior_target14: i32,
    #[serde(rename = "behaviorTarget15")]
    pub behavior_target15: i32,
    #[serde(rename = "behaviorTarget16")]
    pub behavior_target16: i32,
    #[serde(rename = "behaviorTarget17")]
    pub behavior_target17: i32,
    #[serde(rename = "behaviorTarget18")]
    pub behavior_target18: i32,
    #[serde(rename = "behaviorTarget19")]
    pub behavior_target19: i32,
    #[serde(rename = "behaviorTarget2")]
    pub behavior_target2: i32,
    #[serde(rename = "behaviorTarget20")]
    pub behavior_target20: i32,
    #[serde(rename = "behaviorTarget3")]
    pub behavior_target3: i32,
    #[serde(rename = "behaviorTarget4")]
    pub behavior_target4: i32,
    #[serde(rename = "behaviorTarget5")]
    pub behavior_target5: i32,
    #[serde(rename = "behaviorTarget6")]
    pub behavior_target6: i32,
    #[serde(rename = "behaviorTarget7")]
    pub behavior_target7: i32,
    #[serde(rename = "behaviorTarget8")]
    pub behavior_target8: i32,
    #[serde(rename = "behaviorTarget9")]
    pub behavior_target9: i32,
    #[serde(rename = "bigSkillPoint")]
    pub big_skill_point: i32,
    #[serde(rename = "clientIgnoreCondition")]
    pub client_ignore_condition: String,
    #[serde(rename = "condition1")]
    pub condition1: String,
    #[serde(rename = "condition10")]
    pub condition10: String,
    #[serde(rename = "condition11")]
    pub condition11: String,
    #[serde(rename = "condition12")]
    pub condition12: String,
    #[serde(rename = "condition13")]
    pub condition13: String,
    #[serde(rename = "condition14")]
    pub condition14: String,
    #[serde(rename = "condition15")]
    pub condition15: String,
    #[serde(rename = "condition16")]
    pub condition16: String,
    #[serde(rename = "condition17")]
    pub condition17: String,
    #[serde(rename = "condition18")]
    pub condition18: String,
    #[serde(rename = "condition19")]
    pub condition19: String,
    #[serde(rename = "condition2")]
    pub condition2: String,
    #[serde(rename = "condition20")]
    pub condition20: String,
    #[serde(rename = "condition3")]
    pub condition3: String,
    #[serde(rename = "condition4")]
    pub condition4: String,
    #[serde(rename = "condition5")]
    pub condition5: String,
    #[serde(rename = "condition6")]
    pub condition6: String,
    #[serde(rename = "condition7")]
    pub condition7: String,
    #[serde(rename = "condition8")]
    pub condition8: String,
    #[serde(rename = "condition9")]
    pub condition9: String,
    #[serde(rename = "conditionTarget1")]
    pub condition_target1: i32,
    #[serde(rename = "conditionTarget10")]
    pub condition_target10: i32,
    #[serde(rename = "conditionTarget11")]
    pub condition_target11: i32,
    #[serde(rename = "conditionTarget12")]
    pub condition_target12: i32,
    #[serde(rename = "conditionTarget13")]
    pub condition_target13: i32,
    #[serde(rename = "conditionTarget14")]
    pub condition_target14: i32,
    #[serde(rename = "conditionTarget15")]
    pub condition_target15: i32,
    #[serde(rename = "conditionTarget16")]
    pub condition_target16: i32,
    #[serde(rename = "conditionTarget17")]
    pub condition_target17: i32,
    #[serde(rename = "conditionTarget18")]
    pub condition_target18: i32,
    #[serde(rename = "conditionTarget19")]
    pub condition_target19: i32,
    #[serde(rename = "conditionTarget2")]
    pub condition_target2: i32,
    #[serde(rename = "conditionTarget20")]
    pub condition_target20: i32,
    #[serde(rename = "conditionTarget3")]
    pub condition_target3: i32,
    #[serde(rename = "conditionTarget4")]
    pub condition_target4: i32,
    #[serde(rename = "conditionTarget5")]
    pub condition_target5: i32,
    #[serde(rename = "conditionTarget6")]
    pub condition_target6: i32,
    #[serde(rename = "conditionTarget7")]
    pub condition_target7: i32,
    #[serde(rename = "conditionTarget8")]
    pub condition_target8: i32,
    #[serde(rename = "conditionTarget9")]
    pub condition_target9: i32,
    #[serde(rename = "damageRate")]
    pub damage_rate: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "effectTag")]
    pub effect_tag: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isBigSkill")]
    pub is_big_skill: i32,
    #[serde(rename = "isExtra")]
    pub is_extra: i32,
    #[serde(rename = "limit1")]
    pub limit1: i32,
    #[serde(rename = "limit10")]
    pub limit10: i32,
    #[serde(rename = "limit11")]
    pub limit11: i32,
    #[serde(rename = "limit12")]
    pub limit12: i32,
    #[serde(rename = "limit13")]
    pub limit13: i32,
    #[serde(rename = "limit14")]
    pub limit14: i32,
    #[serde(rename = "limit15")]
    pub limit15: i32,
    #[serde(rename = "limit16")]
    pub limit16: i32,
    #[serde(rename = "limit17")]
    pub limit17: i32,
    #[serde(rename = "limit18")]
    pub limit18: i32,
    #[serde(rename = "limit19")]
    pub limit19: i32,
    #[serde(rename = "limit2")]
    pub limit2: i32,
    #[serde(rename = "limit20")]
    pub limit20: i32,
    #[serde(rename = "limit3")]
    pub limit3: i32,
    #[serde(rename = "limit4")]
    pub limit4: i32,
    #[serde(rename = "limit5")]
    pub limit5: i32,
    #[serde(rename = "limit6")]
    pub limit6: i32,
    #[serde(rename = "limit7")]
    pub limit7: i32,
    #[serde(rename = "limit8")]
    pub limit8: i32,
    #[serde(rename = "limit9")]
    pub limit9: i32,
    #[serde(rename = "logicTarget")]
    pub logic_target: i32,
    #[serde(rename = "needExPoint")]
    pub need_ex_point: i32,
    #[serde(rename = "powerCost")]
    pub power_cost: String,
    #[serde(rename = "resistancesTag")]
    pub resistances_tag: String,
    #[serde(rename = "roundLimit1")]
    pub round_limit1: i32,
    #[serde(rename = "roundLimit10")]
    pub round_limit10: i32,
    #[serde(rename = "roundLimit11")]
    pub round_limit11: i32,
    #[serde(rename = "roundLimit12")]
    pub round_limit12: i32,
    #[serde(rename = "roundLimit13")]
    pub round_limit13: i32,
    #[serde(rename = "roundLimit14")]
    pub round_limit14: i32,
    #[serde(rename = "roundLimit15")]
    pub round_limit15: i32,
    #[serde(rename = "roundLimit16")]
    pub round_limit16: i32,
    #[serde(rename = "roundLimit17")]
    pub round_limit17: i32,
    #[serde(rename = "roundLimit18")]
    pub round_limit18: i32,
    #[serde(rename = "roundLimit19")]
    pub round_limit19: i32,
    #[serde(rename = "roundLimit2")]
    pub round_limit2: i32,
    #[serde(rename = "roundLimit20")]
    pub round_limit20: i32,
    #[serde(rename = "roundLimit3")]
    pub round_limit3: i32,
    #[serde(rename = "roundLimit4")]
    pub round_limit4: i32,
    #[serde(rename = "roundLimit5")]
    pub round_limit5: i32,
    #[serde(rename = "roundLimit6")]
    pub round_limit6: i32,
    #[serde(rename = "roundLimit7")]
    pub round_limit7: i32,
    #[serde(rename = "roundLimit8")]
    pub round_limit8: i32,
    #[serde(rename = "roundLimit9")]
    pub round_limit9: i32,
    #[serde(rename = "showTag")]
    pub show_tag: i32,
    #[serde(rename = "skillEffectType")]
    pub skill_effect_type: i32,
    #[serde(rename = "targetLimit")]
    pub target_limit: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct SkillEffectTable {
    records: Vec<SkillEffect>,
    by_id: HashMap<i32, usize>,
}

impl SkillEffectTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SkillEffect> = if let Some(array) = value.as_array() {
            if array.len() >= 2 && array[1].is_array() {
                // Format: ["table_name", [records]]
                serde_json::from_value(array[1].clone())?
            } else {
                // Format: [records]
                serde_json::from_value(value)?
            }
        } else {
            serde_json::from_value(value)?
        };
        
        let mut by_id = HashMap::with_capacity(records.len());
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
        }
        
        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&SkillEffect> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SkillEffect] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SkillEffect> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
