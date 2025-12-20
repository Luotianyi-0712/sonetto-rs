// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoChessSkill {
    #[serde(rename = "condition1")]
    pub condition1: String,
    #[serde(rename = "condition2")]
    pub condition2: String,
    #[serde(rename = "condition3")]
    pub condition3: String,
    #[serde(rename = "countdown")]
    pub countdown: i32,
    #[serde(rename = "downline")]
    pub downline: bool,
    #[serde(rename = "effect1")]
    pub effect1: String,
    #[serde(rename = "effect2")]
    pub effect2: String,
    #[serde(rename = "effect3")]
    pub effect3: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "roundTriggerLimit1")]
    pub round_trigger_limit1: i32,
    #[serde(rename = "roundTriggerLimit2")]
    pub round_trigger_limit2: i32,
    #[serde(rename = "roundTriggerLimit3")]
    pub round_trigger_limit3: i32,
    #[serde(rename = "skillaction")]
    pub skillaction: String,
    #[serde(rename = "skilleffID")]
    pub skilleff_i_d: String,
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(rename = "totalTriggerLimit1")]
    pub total_trigger_limit1: i32,
    #[serde(rename = "totalTriggerLimit2")]
    pub total_trigger_limit2: i32,
    #[serde(rename = "totalTriggerLimit3")]
    pub total_trigger_limit3: i32,
    #[serde(rename = "triggerPoint1")]
    pub trigger_point1: i32,
    #[serde(rename = "triggerPoint2")]
    pub trigger_point2: i32,
    #[serde(rename = "triggerPoint3")]
    pub trigger_point3: i32,
    #[serde(rename = "useeffect")]
    pub useeffect: i32,
}

pub struct AutoChessSkillTable {
    records: Vec<AutoChessSkill>,
    by_id: HashMap<i32, usize>,
}

impl AutoChessSkillTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<AutoChessSkill> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&AutoChessSkill> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[AutoChessSkill] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, AutoChessSkill> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
