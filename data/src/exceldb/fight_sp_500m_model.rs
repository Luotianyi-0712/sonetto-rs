// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightSp500mModel {
    #[serde(rename = "behind")]
    pub behind: String,
    #[serde(rename = "center")]
    pub center: String,
    #[serde(rename = "front")]
    pub front: String,
    #[serde(rename = "headIcon")]
    pub head_icon: String,
    #[serde(rename = "headIconName")]
    pub head_icon_name: String,
    #[serde(rename = "monsterId")]
    pub monster_id: i32,
}

pub struct FightSp500mModelTable {
    records: Vec<FightSp500mModel>,
}

impl FightSp500mModelTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<FightSp500mModel> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[FightSp500mModel] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightSp500mModel> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
