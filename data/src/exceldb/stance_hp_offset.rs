// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StanceHpOffset {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "offsetPos1")]
    pub offset_pos1: Vec<serde_json::Value>,
    #[serde(rename = "offsetPos2")]
    pub offset_pos2: Option<serde_json::Value>,
    #[serde(rename = "offsetPos3")]
    pub offset_pos3: Option<serde_json::Value>,
    #[serde(rename = "offsetPos4")]
    pub offset_pos4: Option<serde_json::Value>,
    #[serde(rename = "offsetPos5")]
    pub offset_pos5: Vec<serde_json::Value>,
    #[serde(rename = "offsetPos6")]
    pub offset_pos6: Vec<serde_json::Value>,
    #[serde(rename = "offsetPos7")]
    pub offset_pos7: Vec<serde_json::Value>,
    #[serde(rename = "offsetPos8")]
    pub offset_pos8: Vec<serde_json::Value>,
}

pub struct StanceHpOffsetTable {
    records: Vec<StanceHpOffset>,
    by_id: HashMap<i32, usize>,
}

impl StanceHpOffsetTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<StanceHpOffset> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&StanceHpOffset> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[StanceHpOffset] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, StanceHpOffset> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
