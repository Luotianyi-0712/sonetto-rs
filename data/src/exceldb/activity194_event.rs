// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity194Event {
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "eventGroup")]
    pub event_group: i32,
    #[serde(rename = "eventId")]
    pub event_id: i32,
    #[serde(rename = "eventType")]
    pub event_type: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "number")]
    pub number: i32,
    #[serde(rename = "optionIds")]
    pub option_ids: String,
    #[serde(rename = "picture")]
    pub picture: String,
    #[serde(rename = "position")]
    pub position: String,
    #[serde(rename = "trigger")]
    pub trigger: String,
}

pub struct Activity194EventTable {
    records: Vec<Activity194Event>,
}

impl Activity194EventTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity194Event> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity194Event] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity194Event> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
