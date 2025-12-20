// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity112 {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "bonus")]
    pub bonus: String,
    #[serde(rename = "chatheadsOffSet")]
    pub chatheads_off_set: String,
    #[serde(rename = "head")]
    pub head: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "items")]
    pub items: String,
    #[serde(rename = "skin")]
    pub skin: String,
    #[serde(rename = "skin2")]
    pub skin2: String,
    #[serde(rename = "skin2OffSet")]
    pub skin2_off_set: String,
    #[serde(rename = "skinOffSet")]
    pub skin_off_set: String,
    #[serde(rename = "storyId")]
    pub story_id: i32,
    #[serde(rename = "theme")]
    pub theme: String,
    #[serde(rename = "theme2")]
    pub theme2: String,
    #[serde(rename = "themeDone")]
    pub theme_done: String,
    #[serde(rename = "themeDone2")]
    pub theme_done2: String,
}

pub struct Activity112Table {
    records: Vec<Activity112>,
    by_id: HashMap<i32, usize>,
}

impl Activity112Table {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity112> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity112> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity112] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity112> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
