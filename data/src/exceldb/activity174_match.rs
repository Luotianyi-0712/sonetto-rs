// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity174Match {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "lostValue")]
    pub lost_value: i32,
    #[serde(rename = "matchRule")]
    pub match_rule: String,
    #[serde(rename = "matchRuleLimit")]
    pub match_rule_limit: String,
    #[serde(rename = "rank")]
    pub rank: i32,
    #[serde(rename = "robotRate")]
    pub robot_rate: String,
    #[serde(rename = "score")]
    pub score: String,
    #[serde(rename = "winValue")]
    pub win_value: i32,
}

pub struct Activity174MatchTable {
    records: Vec<Activity174Match>,
}

impl Activity174MatchTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity174Match> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity174Match] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity174Match> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
