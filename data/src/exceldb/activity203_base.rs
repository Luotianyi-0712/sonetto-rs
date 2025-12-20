// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity203Base {
    #[serde(rename = "baseId")]
    pub base_id: i32,
    #[serde(rename = "dispatchInterval")]
    pub dispatch_interval: i32,
    #[serde(rename = "initialCamp")]
    pub initial_camp: i32,
    #[serde(rename = "initialSoldier")]
    pub initial_soldier: i32,
    #[serde(rename = "isHQ")]
    pub is_h_q: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "picture")]
    pub picture: String,
    #[serde(rename = "soldierLimit")]
    pub soldier_limit: i32,
    #[serde(rename = "soldierRecover")]
    pub soldier_recover: i32,
    #[serde(rename = "type")]
    pub r#type: String,
}

pub struct Activity203BaseTable {
    records: Vec<Activity203Base>,
}

impl Activity203BaseTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity203Base> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity203Base] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity203Base> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
