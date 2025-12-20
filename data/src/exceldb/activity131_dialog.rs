// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity131Dialog {
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "option_param")]
    pub option_param: String,
    #[serde(rename = "param")]
    pub param: String,
    #[serde(rename = "single")]
    pub single: i32,
    #[serde(rename = "speaker")]
    pub speaker: String,
    #[serde(rename = "stepId")]
    pub step_id: i32,
    #[serde(rename = "type")]
    pub r#type: String,
}

pub struct Activity131DialogTable {
    records: Vec<Activity131Dialog>,
    by_id: HashMap<i32, usize>,
}

impl Activity131DialogTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity131Dialog> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity131Dialog> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity131Dialog] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity131Dialog> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
