// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity163Clue {
    #[serde(rename = "clueDesc")]
    pub clue_desc: String,
    #[serde(rename = "clueIcon")]
    pub clue_icon: String,
    #[serde(rename = "clueId")]
    pub clue_id: i32,
    #[serde(rename = "clueName")]
    pub clue_name: String,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "materialId")]
    pub material_id: String,
    #[serde(rename = "replaceId")]
    pub replace_id: i32,
}

pub struct Activity163ClueTable {
    records: Vec<Activity163Clue>,
}

impl Activity163ClueTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity163Clue> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity163Clue] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity163Clue> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
