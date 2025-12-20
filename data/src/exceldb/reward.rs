// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    #[serde(rename = "dailyDrop")]
    pub daily_drop: i32,
    #[serde(rename = "dailyGainWarning")]
    pub daily_gain_warning: i32,
    #[serde(rename = "rewardGroup1")]
    pub reward_group1: String,
    #[serde(rename = "rewardGroup2")]
    pub reward_group2: String,
    #[serde(rename = "rewardGroup3")]
    pub reward_group3: String,
    #[serde(rename = "rewardGroup4")]
    pub reward_group4: String,
    #[serde(rename = "rewardGroup5")]
    pub reward_group5: String,
    #[serde(rename = "rewardGroup6")]
    pub reward_group6: String,
    #[serde(rename = "rewardGroup7")]
    pub reward_group7: String,
    #[serde(rename = "rewardGroup8")]
    pub reward_group8: String,
    #[serde(rename = "reward_id")]
    pub reward_id: i32,
}

pub struct RewardTable {
    records: Vec<Reward>,
}

impl RewardTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Reward> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Reward] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Reward> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
