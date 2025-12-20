// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity178Episode {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "condition")]
    pub condition: i32,
    #[serde(rename = "condition2")]
    pub condition2: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "longDesc")]
    pub long_desc: String,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "reward")]
    pub reward: String,
    #[serde(rename = "shortDesc")]
    pub short_desc: String,
    #[serde(rename = "target")]
    pub target: String,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct Activity178EpisodeTable {
    records: Vec<Activity178Episode>,
    by_id: HashMap<i32, usize>,
}

impl Activity178EpisodeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity178Episode> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity178Episode> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity178Episode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity178Episode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
