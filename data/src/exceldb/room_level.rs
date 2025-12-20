// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomLevel {
    #[serde(rename = "characterLimit")]
    pub character_limit: i32,
    #[serde(rename = "cost")]
    pub cost: String,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "maxBlockCount")]
    pub max_block_count: i32,
    #[serde(rename = "needBlockCount")]
    pub need_block_count: i32,
    #[serde(rename = "needCost")]
    pub need_cost: String,
    #[serde(rename = "needEpisode")]
    pub need_episode: i32,
}

pub struct RoomLevelTable {
    records: Vec<RoomLevel>,
}

impl RoomLevelTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RoomLevel> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[RoomLevel] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomLevel> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
