// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RougeRewardStage {
    #[serde(rename = "bigRewardId")]
    pub big_reward_id: i32,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "jump")]
    pub jump: String,
    #[serde(rename = "lockName")]
    pub lock_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "openTime")]
    pub open_time: String,
    #[serde(rename = "pointLimit")]
    pub point_limit: i32,
    #[serde(rename = "preStage")]
    pub pre_stage: i32,
    #[serde(rename = "season")]
    pub season: i32,
    #[serde(rename = "stage")]
    pub stage: i32,
}

pub struct RougeRewardStageTable {
    records: Vec<RougeRewardStage>,
}

impl RougeRewardStageTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RougeRewardStage> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[RougeRewardStage] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RougeRewardStage> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
