// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoldierSkill {
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "condition")]
    pub condition: String,
    #[serde(rename = "effect")]
    pub effect: String,
    #[serde(rename = "growUpTime")]
    pub grow_up_time: i32,
    #[serde(rename = "growUploop")]
    pub grow_uploop: bool,
    #[serde(rename = "roundTriggerCountLimit")]
    pub round_trigger_count_limit: i32,
    #[serde(rename = "skillDes")]
    pub skill_des: String,
    #[serde(rename = "skillId")]
    pub skill_id: i32,
    #[serde(rename = "skillName")]
    pub skill_name: String,
    #[serde(rename = "totalTriggerCountLimit")]
    pub total_trigger_count_limit: i32,
    #[serde(rename = "triggerPoint")]
    pub trigger_point: i32,
    #[serde(rename = "type")]
    pub r#type: String,
}

pub struct SoldierSkillTable {
    records: Vec<SoldierSkill>,
}

impl SoldierSkillTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SoldierSkill> = if let Some(array) = value.as_array() {
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
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[SoldierSkill] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SoldierSkill> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
