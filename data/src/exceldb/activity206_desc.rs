// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity206Desc {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "ruleDesc")]
    pub rule_desc: String,
    #[serde(rename = "ruleTitle")]
    pub rule_title: String,
    #[serde(rename = "stageId")]
    pub stage_id: i32,
    #[serde(rename = "targetDesc")]
    pub target_desc: String,
}

pub struct Activity206DescTable {
    records: Vec<Activity206Desc>,
}

impl Activity206DescTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity206Desc> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity206Desc] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity206Desc> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
