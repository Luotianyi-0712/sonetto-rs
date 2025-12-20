// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity188Ai {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "difficult")]
    pub difficult: i32,
    #[serde(rename = "pairRate")]
    pub pair_rate: i32,
    #[serde(rename = "pairRound")]
    pub pair_round: i32,
    #[serde(rename = "reverseTime")]
    pub reverse_time: i32,
}

pub struct Activity188AiTable {
    records: Vec<Activity188Ai>,
}

impl Activity188AiTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity188Ai> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity188Ai] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity188Ai> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
