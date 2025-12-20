// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity181Box {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "bonus")]
    pub bonus: String,
    #[serde(rename = "obtainEnd")]
    pub obtain_end: String,
    #[serde(rename = "obtainStart")]
    pub obtain_start: String,
    #[serde(rename = "obtainTimes")]
    pub obtain_times: i32,
    #[serde(rename = "obtainType")]
    pub obtain_type: i32,
    #[serde(rename = "showOfflineTime")]
    pub show_offline_time: String,
    #[serde(rename = "showOnlineTime")]
    pub show_online_time: String,
    #[serde(rename = "totalBox")]
    pub total_box: i32,
}

pub struct Activity181BoxTable {
    records: Vec<Activity181Box>,
}

impl Activity181BoxTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity181Box> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity181Box] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity181Box> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
