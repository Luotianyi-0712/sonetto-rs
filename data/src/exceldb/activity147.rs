// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity147 {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "descList")]
    pub desc_list: String,
    #[serde(rename = "dialogs")]
    pub dialogs: String,
    #[serde(rename = "jumpId")]
    pub jump_id: i32,
    #[serde(rename = "rewardList")]
    pub reward_list: String,
    #[serde(rename = "spineRes")]
    pub spine_res: String,
}

pub struct Activity147Table {
    records: Vec<Activity147>,
}

impl Activity147Table {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity147> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity147] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity147> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
