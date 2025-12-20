// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerLevel {
    #[serde(rename = "addBuyRecoverPower")]
    pub add_buy_recover_power: i32,
    #[serde(rename = "addUpRecoverPower")]
    pub add_up_recover_power: i32,
    #[serde(rename = "bonus")]
    pub bonus: i32,
    #[serde(rename = "exp")]
    pub exp: i32,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "maxAutoRecoverPower")]
    pub max_auto_recover_power: i32,
}

pub struct PlayerLevelTable {
    records: Vec<PlayerLevel>,
}

impl PlayerLevelTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<PlayerLevel> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[PlayerLevel] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, PlayerLevel> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
