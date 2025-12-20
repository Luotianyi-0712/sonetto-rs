// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDestinyFacetsConsume {
    #[serde(rename = "consume")]
    pub consume: String,
    #[serde(rename = "facetsId")]
    pub facets_id: i32,
    #[serde(rename = "facetsSort")]
    pub facets_sort: i32,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "keyword")]
    pub keyword: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "tend")]
    pub tend: i32,
}

pub struct CharacterDestinyFacetsConsumeTable {
    records: Vec<CharacterDestinyFacetsConsume>,
}

impl CharacterDestinyFacetsConsumeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<CharacterDestinyFacetsConsume> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[CharacterDestinyFacetsConsume] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CharacterDestinyFacetsConsume> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
