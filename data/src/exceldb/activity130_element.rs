// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity130Element {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "elementId")]
    pub element_id: i32,
    #[serde(rename = "param")]
    pub param: String,
    #[serde(rename = "res")]
    pub res: String,
    #[serde(rename = "skipFinish")]
    pub skip_finish: i32,
    #[serde(rename = "type")]
    pub r#type: String,
}

pub struct Activity130ElementTable {
    records: Vec<Activity130Element>,
}

impl Activity130ElementTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity130Element> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity130Element] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity130Element> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
