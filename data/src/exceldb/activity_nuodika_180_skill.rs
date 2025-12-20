// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityNuodika180Skill {
    #[serde(rename = "effect")]
    pub effect: i32,
    #[serde(rename = "param")]
    pub param: String,
    #[serde(rename = "scale")]
    pub scale: String,
    #[serde(rename = "skillId")]
    pub skill_id: i32,
    #[serde(rename = "trigger")]
    pub trigger: String,
}

pub struct ActivityNuodika180SkillTable {
    records: Vec<ActivityNuodika180Skill>,
}

impl ActivityNuodika180SkillTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<ActivityNuodika180Skill> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[ActivityNuodika180Skill] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, ActivityNuodika180Skill> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
