// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentStyle {
    #[serde(rename = "color")]
    pub color: String,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "replaceCube")]
    pub replace_cube: String,
    #[serde(rename = "styleId")]
    pub style_id: i32,
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(rename = "tagicon")]
    pub tagicon: String,
    #[serde(rename = "talentMould")]
    pub talent_mould: i32,
}

pub struct TalentStyleTable {
    records: Vec<TalentStyle>,
}

impl TalentStyleTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<TalentStyle> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[TalentStyle] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TalentStyle> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
