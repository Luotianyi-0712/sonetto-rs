// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity165Ending {
    #[serde(rename = "belongStoryId")]
    pub belong_story_id: i32,
    #[serde(rename = "endingId")]
    pub ending_id: i32,
    #[serde(rename = "endingText")]
    pub ending_text: String,
    #[serde(rename = "finalStepId")]
    pub final_step_id: i32,
    #[serde(rename = "level")]
    pub level: String,
    #[serde(rename = "pic")]
    pub pic: String,
    #[serde(rename = "text")]
    pub text: String,
}

pub struct Activity165EndingTable {
    records: Vec<Activity165Ending>,
}

impl Activity165EndingTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity165Ending> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity165Ending] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity165Ending> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
