// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkageActivity {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "desc1")]
    pub desc1: String,
    #[serde(rename = "desc2")]
    pub desc2: String,
    #[serde(rename = "item1")]
    pub item1: String,
    #[serde(rename = "item2")]
    pub item2: String,
    #[serde(rename = "res_video1")]
    pub res_video1: String,
    #[serde(rename = "res_video2")]
    pub res_video2: String,
    #[serde(rename = "showOfflineTime")]
    pub show_offline_time: String,
    #[serde(rename = "showOnlineTime")]
    pub show_online_time: String,
    #[serde(rename = "systemJumpCode")]
    pub system_jump_code: String,
}

pub struct LinkageActivityTable {
    records: Vec<LinkageActivity>,
}

impl LinkageActivityTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<LinkageActivity> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[LinkageActivity] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, LinkageActivity> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
