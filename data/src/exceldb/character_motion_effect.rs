// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterMotionEffect {
    #[serde(rename = "effectCompName")]
    pub effect_comp_name: String,
    #[serde(rename = "everNode")]
    pub ever_node: String,
    #[serde(rename = "heroResName")]
    pub hero_res_name: String,
    #[serde(rename = "motion")]
    pub motion: String,
    #[serde(rename = "node")]
    pub node: String,
}

pub struct CharacterMotionEffectTable {
    records: Vec<CharacterMotionEffect>,
}

impl CharacterMotionEffectTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<CharacterMotionEffect> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[CharacterMotionEffect] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CharacterMotionEffect> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
