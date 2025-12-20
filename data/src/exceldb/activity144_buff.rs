// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity144Buff {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "buffId")]
    pub buff_id: i32,
    #[serde(rename = "effectDesc")]
    pub effect_desc: String,
    #[serde(rename = "effectParam")]
    pub effect_param: String,
    #[serde(rename = "effectType")]
    pub effect_type: i32,
    #[serde(rename = "reduction")]
    pub reduction: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct Activity144BuffTable {
    records: Vec<Activity144Buff>,
}

impl Activity144BuffTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity144Buff> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity144Buff] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity144Buff> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
