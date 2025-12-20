// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainUiEagle {
    #[serde(rename = "animName")]
    pub anim_name: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isSpineAnim")]
    pub is_spine_anim: i32,
    #[serde(rename = "isoutline")]
    pub isoutline: i32,
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "loop")]
    pub r#loop: i32,
    #[serde(rename = "odds_nextstep")]
    pub odds_nextstep: String,
    #[serde(rename = "option_nextstep")]
    pub option_nextstep: i32,
    #[serde(rename = "playfadeAnim")]
    pub playfade_anim: String,
    #[serde(rename = "times")]
    pub times: String,
}

pub struct MainUiEagleTable {
    records: Vec<MainUiEagle>,
    by_id: HashMap<i32, usize>,
}

impl MainUiEagleTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<MainUiEagle> = if let Some(array) = value.as_array() {
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
        
        let mut by_id = HashMap::with_capacity(records.len());
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
        }
        
        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&MainUiEagle> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[MainUiEagle] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, MainUiEagle> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
