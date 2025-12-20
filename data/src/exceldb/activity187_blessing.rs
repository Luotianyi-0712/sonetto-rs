// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity187Blessing {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "blessing")]
    pub blessing: String,
    #[serde(rename = "bonus")]
    pub bonus: String,
    #[serde(rename = "lantern")]
    pub lantern: String,
    #[serde(rename = "lanternImg")]
    pub lantern_img: String,
    #[serde(rename = "lanternImgBg")]
    pub lantern_img_bg: String,
    #[serde(rename = "lanternRibbon")]
    pub lantern_ribbon: String,
}

pub struct Activity187BlessingTable {
    records: Vec<Activity187Blessing>,
}

impl Activity187BlessingTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity187Blessing> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity187Blessing] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity187Blessing> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
