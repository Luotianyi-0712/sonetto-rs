// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentMould {
    #[serde(rename = "allShape")]
    pub all_shape: String,
    #[serde(rename = "talentId")]
    pub talent_id: i32,
    #[serde(rename = "talentMould")]
    pub talent_mould: i32,
    #[serde(rename = "type10")]
    pub type10: String,
    #[serde(rename = "type11")]
    pub type11: String,
    #[serde(rename = "type12")]
    pub type12: String,
    #[serde(rename = "type13")]
    pub type13: String,
    #[serde(rename = "type14")]
    pub type14: String,
    #[serde(rename = "type15")]
    pub type15: String,
    #[serde(rename = "type16")]
    pub type16: String,
    #[serde(rename = "type17")]
    pub type17: String,
    #[serde(rename = "type18")]
    pub type18: String,
    #[serde(rename = "type19")]
    pub type19: String,
    #[serde(rename = "type20")]
    pub type20: String,
}

pub struct TalentMouldTable {
    records: Vec<TalentMould>,
}

impl TalentMouldTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<TalentMould> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[TalentMould] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TalentMould> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
