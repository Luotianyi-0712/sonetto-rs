// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExploreHeroEffect {
    #[serde(rename = "audioId")]
    pub audio_id: i32,
    #[serde(rename = "effectPath")]
    pub effect_path: String,
    #[serde(rename = "hangPath")]
    pub hang_path: String,
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "status")]
    pub status: i32,
}

pub struct ExploreHeroEffectTable {
    records: Vec<ExploreHeroEffect>,
}

impl ExploreHeroEffectTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<ExploreHeroEffect> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[ExploreHeroEffect] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, ExploreHeroEffect> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
