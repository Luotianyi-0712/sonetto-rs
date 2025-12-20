// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenGroup {
    #[serde(rename = "hero_number")]
    pub hero_number: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "lock_desc")]
    pub lock_desc: i32,
    #[serde(rename = "need_enter_episode")]
    pub need_enter_episode: i32,
    #[serde(rename = "need_episode")]
    pub need_episode: i32,
    #[serde(rename = "need_finish_guide")]
    pub need_finish_guide: i32,
    #[serde(rename = "need_level")]
    pub need_level: i32,
    #[serde(rename = "showInEpisode")]
    pub show_in_episode: i32,
}

pub struct OpenGroupTable {
    records: Vec<OpenGroup>,
    by_id: HashMap<i32, usize>,
}

impl OpenGroupTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<OpenGroup> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&OpenGroup> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[OpenGroup] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, OpenGroup> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
