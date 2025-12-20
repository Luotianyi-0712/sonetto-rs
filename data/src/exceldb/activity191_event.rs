// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity191Event {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "offset")]
    pub offset: String,
    #[serde(rename = "outDesc")]
    pub out_desc: String,
    #[serde(rename = "rewardView")]
    pub reward_view: String,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
    #[serde(rename = "stage")]
    pub stage: i32,
    #[serde(rename = "task")]
    pub task: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

pub struct Activity191EventTable {
    records: Vec<Activity191Event>,
    by_id: HashMap<i32, usize>,
}

impl Activity191EventTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity191Event> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity191Event> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity191Event] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity191Event> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
