// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerAssistDevelop {
    #[serde(rename = "attribute")]
    pub attribute: String,
    #[serde(rename = "bossId")]
    pub boss_id: i32,
    #[serde(rename = "extraRule")]
    pub extra_rule: String,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "passiveSkills")]
    pub passive_skills: String,
    #[serde(rename = "talentPoint")]
    pub talent_point: i32,
}

pub struct TowerAssistDevelopTable {
    records: Vec<TowerAssistDevelop>,
}

impl TowerAssistDevelopTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<TowerAssistDevelop> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[TowerAssistDevelop] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerAssistDevelop> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
