// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RougeGeniusBranch {
    #[serde(rename = "attribute")]
    pub attribute: String,
    #[serde(rename = "before")]
    pub before: String,
    #[serde(rename = "cost")]
    pub cost: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "effects")]
    pub effects: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "initialCollection")]
    pub initial_collection: String,
    #[serde(rename = "isOrigin")]
    pub is_origin: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "openDesc")]
    pub open_desc: String,
    #[serde(rename = "pos")]
    pub pos: String,
    #[serde(rename = "season")]
    pub season: i32,
    #[serde(rename = "show")]
    pub show: i32,
    #[serde(rename = "startView")]
    pub start_view: String,
    #[serde(rename = "talent")]
    pub talent: i32,
}

pub struct RougeGeniusBranchTable {
    records: Vec<RougeGeniusBranch>,
    by_id: HashMap<i32, usize>,
}

impl RougeGeniusBranchTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RougeGeniusBranch> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&RougeGeniusBranch> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[RougeGeniusBranch] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RougeGeniusBranch> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
