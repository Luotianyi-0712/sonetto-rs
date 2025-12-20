// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerAssistBossChange {
    #[serde(rename = "activeSkills")]
    pub active_skills: String,
    #[serde(rename = "bossId")]
    pub boss_id: i32,
    #[serde(rename = "coldTime")]
    pub cold_time: i32,
    #[serde(rename = "form")]
    pub form: i32,
    #[serde(rename = "passiveSkills")]
    pub passive_skills: String,
    #[serde(rename = "replacePassiveSkills")]
    pub replace_passive_skills: String,
    #[serde(rename = "resMaxVal")]
    pub res_max_val: i32,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
}

pub struct TowerAssistBossChangeTable {
    records: Vec<TowerAssistBossChange>,
}

impl TowerAssistBossChangeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<TowerAssistBossChange> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[TowerAssistBossChange] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerAssistBossChange> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
