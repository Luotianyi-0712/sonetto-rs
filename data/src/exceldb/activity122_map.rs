// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity122Map {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "audioAmbient")]
    pub audio_ambient: i32,
    #[serde(rename = "bgPath")]
    pub bg_path: String,
    #[serde(rename = "decorateObjects")]
    pub decorate_objects: String,
    #[serde(rename = "height")]
    pub height: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "objects")]
    pub objects: String,
    #[serde(rename = "offset")]
    pub offset: String,
    #[serde(rename = "tilebase")]
    pub tilebase: String,
    #[serde(rename = "width")]
    pub width: i32,
}

pub struct Activity122MapTable {
    records: Vec<Activity122Map>,
    by_id: HashMap<i32, usize>,
}

impl Activity122MapTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity122Map> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity122Map> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity122Map] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity122Map> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
