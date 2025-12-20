// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity123Stage {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "finalPos")]
    pub final_pos: String,
    #[serde(rename = "finalScale")]
    pub final_scale: String,
    #[serde(rename = "initPos")]
    pub init_pos: String,
    #[serde(rename = "initScale")]
    pub init_scale: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "preCondition")]
    pub pre_condition: String,
    #[serde(rename = "recommend")]
    pub recommend: String,
    #[serde(rename = "recommendSchool")]
    pub recommend_school: String,
    #[serde(rename = "res")]
    pub res: String,
    #[serde(rename = "stage")]
    pub stage: i32,
    #[serde(rename = "stageCondition")]
    pub stage_condition: String,
}

pub struct Activity123StageTable {
    records: Vec<Activity123Stage>,
}

impl Activity123StageTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity123Stage> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity123Stage] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity123Stage> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
