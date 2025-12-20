// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stance {
    #[serde(rename = "cardCamera")]
    pub card_camera: String,
    #[serde(rename = "dec_stance")]
    pub dec_stance: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "pos1")]
    pub pos1: Option<serde_json::Value>,
    #[serde(rename = "pos2")]
    pub pos2: Option<serde_json::Value>,
    #[serde(rename = "pos3")]
    pub pos3: Option<serde_json::Value>,
    #[serde(rename = "pos4")]
    pub pos4: Option<serde_json::Value>,
    #[serde(rename = "pos5")]
    pub pos5: Option<serde_json::Value>,
    #[serde(rename = "pos6")]
    pub pos6: Option<serde_json::Value>,
    #[serde(rename = "pos7")]
    pub pos7: Option<serde_json::Value>,
    #[serde(rename = "pos8")]
    pub pos8: Option<serde_json::Value>,
    #[serde(rename = "subPos1")]
    pub sub_pos1: Option<serde_json::Value>,
    #[serde(rename = "subPos2")]
    pub sub_pos2: Option<serde_json::Value>,
    #[serde(rename = "subPos3")]
    pub sub_pos3: Vec<serde_json::Value>,
}

pub struct StanceTable {
    records: Vec<Stance>,
    by_id: HashMap<i32, usize>,
}

impl StanceTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Stance> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Stance> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Stance] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Stance> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
