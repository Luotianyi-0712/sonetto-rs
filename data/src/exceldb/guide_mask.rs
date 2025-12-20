// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuideMask {
    #[serde(rename = "goPath1")]
    pub go_path1: String,
    #[serde(rename = "goPath2")]
    pub go_path2: String,
    #[serde(rename = "goPath3")]
    pub go_path3: String,
    #[serde(rename = "goPath4")]
    pub go_path4: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "uiInfo1")]
    pub ui_info1: String,
    #[serde(rename = "uiInfo2")]
    pub ui_info2: String,
    #[serde(rename = "uiInfo3")]
    pub ui_info3: String,
    #[serde(rename = "uiInfo4")]
    pub ui_info4: String,
    #[serde(rename = "uiOffset1")]
    pub ui_offset1: String,
    #[serde(rename = "uiOffset2")]
    pub ui_offset2: String,
    #[serde(rename = "uiOffset3")]
    pub ui_offset3: String,
    #[serde(rename = "uiOffset4")]
    pub ui_offset4: String,
}

pub struct GuideMaskTable {
    records: Vec<GuideMask>,
    by_id: HashMap<i32, usize>,
}

impl GuideMaskTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<GuideMask> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&GuideMask> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[GuideMask] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, GuideMask> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
