// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity166BaseTarget {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "baseId")]
    pub base_id: i32,
    #[serde(rename = "score")]
    pub score: String,
    #[serde(rename = "targetDesc")]
    pub target_desc: String,
    #[serde(rename = "targetId")]
    pub target_id: i32,
    #[serde(rename = "targetParam")]
    pub target_param: String,
    #[serde(rename = "targetType")]
    pub target_type: i32,
}

pub struct Activity166BaseTargetTable {
    records: Vec<Activity166BaseTarget>,
}

impl Activity166BaseTargetTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity166BaseTarget> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity166BaseTarget] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity166BaseTarget> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
