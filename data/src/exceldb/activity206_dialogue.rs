// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity206Dialogue {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "chaseId")]
    pub chase_id: i32,
    #[serde(rename = "dialog")]
    pub dialog: String,
    #[serde(rename = "roleIcon")]
    pub role_icon: String,
    #[serde(rename = "roleName")]
    pub role_name: String,
    #[serde(rename = "roleNameEn")]
    pub role_name_en: String,
}

pub struct Activity206DialogueTable {
    records: Vec<Activity206Dialogue>,
}

impl Activity206DialogueTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity206Dialogue> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity206Dialogue] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity206Dialogue> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
