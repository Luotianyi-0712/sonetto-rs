// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Act139SubHeroTask {
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "descSuffix")]
    pub desc_suffix: String,
    #[serde(rename = "elementIds")]
    pub element_ids: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "lockDesc")]
    pub lock_desc: String,
    #[serde(rename = "reward")]
    pub reward: String,
    #[serde(rename = "storyId")]
    pub story_id: i32,
    #[serde(rename = "taskId")]
    pub task_id: i32,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "unlockParam")]
    pub unlock_param: String,
    #[serde(rename = "unlockType")]
    pub unlock_type: i32,
}

pub struct Act139SubHeroTaskTable {
    records: Vec<Act139SubHeroTask>,
    by_id: HashMap<i32, usize>,
}

impl Act139SubHeroTaskTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Act139SubHeroTask> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Act139SubHeroTask> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Act139SubHeroTask] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Act139SubHeroTask> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
