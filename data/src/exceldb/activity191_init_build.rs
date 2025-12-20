// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity191InitBuild {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "coin")]
    pub coin: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "hero")]
    pub hero: String,
    #[serde(rename = "item")]
    pub item: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "randHero")]
    pub rand_hero: String,
    #[serde(rename = "randItem")]
    pub rand_item: String,
    #[serde(rename = "rewardHero")]
    pub reward_hero: String,
    #[serde(rename = "rewardItem")]
    pub reward_item: String,
    #[serde(rename = "style")]
    pub style: i32,
}

pub struct Activity191InitBuildTable {
    records: Vec<Activity191InitBuild>,
}

impl Activity191InitBuildTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity191InitBuild> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity191InitBuild] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity191InitBuild> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
