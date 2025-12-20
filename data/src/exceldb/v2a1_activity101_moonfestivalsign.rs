// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V2a1Activity101Moonfestivalsign {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "day")]
    pub day: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "titile")]
    pub titile: String,
}

pub struct V2a1Activity101MoonfestivalsignTable {
    records: Vec<V2a1Activity101Moonfestivalsign>,
}

impl V2a1Activity101MoonfestivalsignTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<V2a1Activity101Moonfestivalsign> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[V2a1Activity101Moonfestivalsign] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, V2a1Activity101Moonfestivalsign> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
