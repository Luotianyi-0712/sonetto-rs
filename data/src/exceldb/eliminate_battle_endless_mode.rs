// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EliminateBattleEndlessMode {
    #[serde(rename = "hpUp")]
    pub hp_up: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "powerUp1")]
    pub power_up1: String,
    #[serde(rename = "powerUp2")]
    pub power_up2: String,
    #[serde(rename = "powerUp3")]
    pub power_up3: String,
    #[serde(rename = "powerUp4")]
    pub power_up4: String,
    #[serde(rename = "powerUp5")]
    pub power_up5: String,
    #[serde(rename = "skill1")]
    pub skill1: String,
    #[serde(rename = "skill2")]
    pub skill2: String,
    #[serde(rename = "skill3")]
    pub skill3: String,
    #[serde(rename = "skill4")]
    pub skill4: String,
    #[serde(rename = "skill5")]
    pub skill5: String,
}

pub struct EliminateBattleEndlessModeTable {
    records: Vec<EliminateBattleEndlessMode>,
    by_id: HashMap<i32, usize>,
}

impl EliminateBattleEndlessModeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<EliminateBattleEndlessMode> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&EliminateBattleEndlessMode> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[EliminateBattleEndlessMode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, EliminateBattleEndlessMode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
