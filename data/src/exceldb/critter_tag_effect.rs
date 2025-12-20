// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CritterTagEffect {
    #[serde(rename = "catalogue")]
    pub catalogue: i32,
    #[serde(rename = "condition")]
    pub condition: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "parameter")]
    pub parameter: String,
    #[serde(rename = "previewCondition")]
    pub preview_condition: String,
    #[serde(rename = "target")]
    pub target: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct CritterTagEffectTable {
    records: Vec<CritterTagEffect>,
    by_id: HashMap<String, usize>,
}

impl CritterTagEffectTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<CritterTagEffect> = if let Some(array) = value.as_array() {
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
            by_id.insert(record.id.clone(), idx);
        }
        
        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: String) -> Option<&CritterTagEffect> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[CritterTagEffect] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CritterTagEffect> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
