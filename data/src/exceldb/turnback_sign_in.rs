// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnbackSignIn {
    #[serde(rename = "bonus")]
    pub bonus: String,
    #[serde(rename = "characterId")]
    pub character_id: i32,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "day")]
    pub day: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "turnbackId")]
    pub turnback_id: i32,
}

pub struct TurnbackSignInTable {
    records: Vec<TurnbackSignIn>,
}

impl TurnbackSignInTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<TurnbackSignIn> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[TurnbackSignIn] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TurnbackSignIn> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
