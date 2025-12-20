// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity194Team {
    #[serde(rename = "buffId")]
    pub buff_id: String,
    #[serde(rename = "iconOffset")]
    pub icon_offset: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "picture")]
    pub picture: String,
    #[serde(rename = "roundActionTime")]
    pub round_action_time: i32,
    #[serde(rename = "teamId")]
    pub team_id: i32,
}

pub struct Activity194TeamTable {
    records: Vec<Activity194Team>,
}

impl Activity194TeamTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity194Team> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity194Team] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity194Team> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
