// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalCopy {
    #[serde(rename = "abPath")]
    pub ab_path: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "effect")]
    pub effect: String,
    #[serde(rename = "effectDesc")]
    pub effect_desc: String,
    #[serde(rename = "enName")]
    pub en_name: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "lightPram")]
    pub light_pram: i32,
    #[serde(rename = "mapGroup")]
    pub map_group: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "pic")]
    pub pic: String,
    #[serde(rename = "useScene")]
    pub use_scene: i32,
}

pub struct SurvivalCopyTable {
    records: Vec<SurvivalCopy>,
    by_id: HashMap<i32, usize>,
}

impl SurvivalCopyTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SurvivalCopy> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SurvivalCopy> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SurvivalCopy] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SurvivalCopy> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
