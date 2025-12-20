// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoChessMasterSkill {
    #[serde(rename = "abilities")]
    pub abilities: String,
    #[serde(rename = "activeChessSkill")]
    pub active_chess_skill: String,
    #[serde(rename = "cost")]
    pub cost: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "needTarget")]
    pub need_target: bool,
    #[serde(rename = "passiveChessSkills")]
    pub passive_chess_skills: String,
    #[serde(rename = "skillIndex")]
    pub skill_index: i32,
    #[serde(rename = "skillaction")]
    pub skillaction: String,
    #[serde(rename = "targetType")]
    pub target_type: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "useeffect")]
    pub useeffect: i32,
}

pub struct AutoChessMasterSkillTable {
    records: Vec<AutoChessMasterSkill>,
    by_id: HashMap<i32, usize>,
}

impl AutoChessMasterSkillTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<AutoChessMasterSkill> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&AutoChessMasterSkill> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[AutoChessMasterSkill] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, AutoChessMasterSkill> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
