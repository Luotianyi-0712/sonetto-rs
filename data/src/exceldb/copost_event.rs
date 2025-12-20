// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopostEvent {
    #[serde(rename = "allTime")]
    pub all_time: i32,
    #[serde(rename = "bonus")]
    pub bonus: String,
    #[serde(rename = "chaId")]
    pub cha_id: Option<serde_json::Value>,
    #[serde(rename = "chaNum")]
    pub cha_num: i32,
    #[serde(rename = "charaProfile")]
    pub chara_profile: String,
    #[serde(rename = "eventCoordinate")]
    pub event_coordinate: Option<serde_json::Value>,
    #[serde(rename = "eventTextId")]
    pub event_text_id: String,
    #[serde(rename = "eventTitleId")]
    pub event_title_id: String,
    #[serde(rename = "eventType")]
    pub event_type: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "needchaText")]
    pub needcha_text: String,
    #[serde(rename = "reduceTime")]
    pub reduce_time: i32,
}

pub struct CopostEventTable {
    records: Vec<CopostEvent>,
    by_id: HashMap<i32, usize>,
}

impl CopostEventTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<CopostEvent> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&CopostEvent> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[CopostEvent] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CopostEvent> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
