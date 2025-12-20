// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoChessRound {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "assess")]
    pub assess: String,
    #[serde(rename = "maxdamage")]
    pub maxdamage: i32,
    #[serde(rename = "previewCost")]
    pub preview_cost: i32,
    #[serde(rename = "round")]
    pub round: i32,
    #[serde(rename = "starReward")]
    pub star_reward: String,
}

pub struct AutoChessRoundTable {
    records: Vec<AutoChessRound>,
}

impl AutoChessRoundTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<AutoChessRound> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[AutoChessRound] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, AutoChessRound> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
