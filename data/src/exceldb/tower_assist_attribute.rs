// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerAssistAttribute {
    #[serde(rename = "attack")]
    pub attack: i32,
    #[serde(rename = "bossId")]
    pub boss_id: i32,
    #[serde(rename = "cri")]
    pub cri: i32,
    #[serde(rename = "criDmg")]
    pub cri_dmg: i32,
    #[serde(rename = "hp")]
    pub hp: i32,
    #[serde(rename = "teamLevel")]
    pub team_level: i32,
}

pub struct TowerAssistAttributeTable {
    records: Vec<TowerAssistAttribute>,
}

impl TowerAssistAttributeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<TowerAssistAttribute> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[TowerAssistAttribute] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerAssistAttribute> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
