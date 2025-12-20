// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RogueField {
    #[serde(rename = "cost")]
    pub cost: i32,
    #[serde(rename = "equipLevel")]
    pub equip_level: i32,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "level4")]
    pub level4: i32,
    #[serde(rename = "level5")]
    pub level5: i32,
    #[serde(rename = "level6")]
    pub level6: i32,
    #[serde(rename = "talentLevel")]
    pub talent_level: i32,
}

pub struct RogueFieldTable {
    records: Vec<RogueField>,
}

impl RogueFieldTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RogueField> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[RogueField] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RogueField> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
