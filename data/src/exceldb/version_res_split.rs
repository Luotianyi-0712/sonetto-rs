// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionResSplit {
    #[serde(rename = "audio")]
    pub audio: Option<serde_json::Value>,
    #[serde(rename = "chapter")]
    pub chapter: Option<serde_json::Value>,
    #[serde(rename = "folderPath")]
    pub folder_path: Vec<serde_json::Value>,
    #[serde(rename = "guide")]
    pub guide: Option<serde_json::Value>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "packName")]
    pub pack_name: String,
    #[serde(rename = "path")]
    pub path: Vec<serde_json::Value>,
    #[serde(rename = "story")]
    pub story: Option<serde_json::Value>,
    #[serde(rename = "uiFolder")]
    pub ui_folder: Option<serde_json::Value>,
    #[serde(rename = "uiPath")]
    pub ui_path: Vec<serde_json::Value>,
    #[serde(rename = "videoPath")]
    pub video_path: Option<serde_json::Value>,
}

pub struct VersionResSplitTable {
    records: Vec<VersionResSplit>,
    by_id: HashMap<i32, usize>,
}

impl VersionResSplitTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<VersionResSplit> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&VersionResSplit> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[VersionResSplit] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, VersionResSplit> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
