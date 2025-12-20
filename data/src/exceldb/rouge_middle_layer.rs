// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RougeMiddleLayer {
    #[serde(rename = "dayOrNight")]
    pub day_or_night: i32,
    #[serde(rename = "empty")]
    pub empty: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "leavePos")]
    pub leave_pos: String,
    #[serde(rename = "leavePosUnlockParam")]
    pub leave_pos_unlock_param: String,
    #[serde(rename = "leavePosUnlockType")]
    pub leave_pos_unlock_type: i32,
    #[serde(rename = "mapRes")]
    pub map_res: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nextLayer")]
    pub next_layer: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "pathPointPos")]
    pub path_point_pos: String,
    #[serde(rename = "pathSelect")]
    pub path_select: String,
    #[serde(rename = "pointPos")]
    pub point_pos: String,
    #[serde(rename = "version")]
    pub version: String,
}

pub struct RougeMiddleLayerTable {
    records: Vec<RougeMiddleLayer>,
    by_id: HashMap<i32, usize>,
}

impl RougeMiddleLayerTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RougeMiddleLayer> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&RougeMiddleLayer> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[RougeMiddleLayer] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RougeMiddleLayer> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
