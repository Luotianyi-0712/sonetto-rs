// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RougeItemStaticDesc {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "item1")]
    pub item1: i32,
    #[serde(rename = "item1_attr")]
    pub item1_attr: String,
    #[serde(rename = "item2")]
    pub item2: i32,
    #[serde(rename = "item2_attr")]
    pub item2_attr: String,
    #[serde(rename = "item3")]
    pub item3: i32,
    #[serde(rename = "item3_attr")]
    pub item3_attr: String,
}

pub struct RougeItemStaticDescTable {
    records: Vec<RougeItemStaticDesc>,
    by_id: HashMap<i32, usize>,
}

impl RougeItemStaticDescTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RougeItemStaticDesc> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&RougeItemStaticDesc> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[RougeItemStaticDesc] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RougeItemStaticDesc> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
