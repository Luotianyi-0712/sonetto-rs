// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actvity186MiniGameReward {
    #[serde(rename = "blessingdes")]
    pub blessingdes: String,
    #[serde(rename = "blessingtitle")]
    pub blessingtitle: String,
    #[serde(rename = "bonus")]
    pub bonus: String,
    #[serde(rename = "prob")]
    pub prob: i32,
    #[serde(rename = "rewardId")]
    pub reward_id: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct Actvity186MiniGameRewardTable {
    records: Vec<Actvity186MiniGameReward>,
}

impl Actvity186MiniGameRewardTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Actvity186MiniGameReward> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Actvity186MiniGameReward] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Actvity186MiniGameReward> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
