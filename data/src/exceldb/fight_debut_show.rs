// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightDebutShow {
    #[serde(rename = "audioId")]
    pub audio_id: i32,
    #[serde(rename = "effect")]
    pub effect: String,
    #[serde(rename = "effectHangPoint")]
    pub effect_hang_point: String,
    #[serde(rename = "effectTime")]
    pub effect_time: i32,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
}

pub struct FightDebutShowTable {
    records: Vec<FightDebutShow>,
}

impl FightDebutShowTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<FightDebutShow> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[FightDebutShow] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightDebutShow> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
