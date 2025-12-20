// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity114Round {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "banButton1")]
    pub ban_button1: String,
    #[serde(rename = "banButton2")]
    pub ban_button2: String,
    #[serde(rename = "day")]
    pub day: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "eventId")]
    pub event_id: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isSkip")]
    pub is_skip: i32,
    #[serde(rename = "preStoryId")]
    pub pre_story_id: String,
    #[serde(rename = "storyId")]
    pub story_id: i32,
    #[serde(rename = "transition")]
    pub transition: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct Activity114RoundTable {
    records: Vec<Activity114Round>,
    by_id: HashMap<i32, usize>,
}

impl Activity114RoundTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity114Round> = if let Some(array) = value.as_array() {
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
        
        let mut by_id = HashMap::with_capacity(records.len());
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
        }
        
        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Activity114Round> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity114Round] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity114Round> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
