// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneCtrl {
    #[serde(rename = "ctrlName")]
    pub ctrl_name: String,
    #[serde(rename = "param1")]
    pub param1: String,
    #[serde(rename = "param2")]
    pub param2: String,
    #[serde(rename = "param3")]
    pub param3: String,
    #[serde(rename = "param4")]
    pub param4: String,
    #[serde(rename = "resName")]
    pub res_name: String,
}

pub struct SceneCtrlTable {
    records: Vec<SceneCtrl>,
}

impl SceneCtrlTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SceneCtrl> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[SceneCtrl] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SceneCtrl> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
