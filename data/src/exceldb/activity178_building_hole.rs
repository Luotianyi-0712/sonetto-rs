// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity178BuildingHole {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "condition")]
    pub condition: i32,
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "pos")]
    pub pos: String,
    #[serde(rename = "size")]
    pub size: i32,
}

pub struct Activity178BuildingHoleTable {
    records: Vec<Activity178BuildingHole>,
}

impl Activity178BuildingHoleTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity178BuildingHole> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity178BuildingHole] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity178BuildingHole> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
