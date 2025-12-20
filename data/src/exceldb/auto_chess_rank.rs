// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoChessRank {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "isShow")]
    pub is_show: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "protection")]
    pub protection: bool,
    #[serde(rename = "rankId")]
    pub rank_id: i32,
    #[serde(rename = "reward")]
    pub reward: i32,
    #[serde(rename = "round2Score")]
    pub round2_score: String,
    #[serde(rename = "score")]
    pub score: i32,
}

pub struct AutoChessRankTable {
    records: Vec<AutoChessRank>,
}

impl AutoChessRankTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<AutoChessRank> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[AutoChessRank] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, AutoChessRank> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
