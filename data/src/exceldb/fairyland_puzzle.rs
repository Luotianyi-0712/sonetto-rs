// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FairylandPuzzle {
    #[serde(rename = "afterTalkId")]
    pub after_talk_id: i32,
    #[serde(rename = "answer")]
    pub answer: String,
    #[serde(rename = "beforeTalkId")]
    pub before_talk_id: i32,
    #[serde(rename = "elementId")]
    pub element_id: i32,
    #[serde(rename = "errorTalkId")]
    pub error_talk_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "storyTalkId")]
    pub story_talk_id: i32,
    #[serde(rename = "successTalkId")]
    pub success_talk_id: i32,
    #[serde(rename = "tipsTalkId")]
    pub tips_talk_id: i32,
}

pub struct FairylandPuzzleTable {
    records: Vec<FairylandPuzzle>,
    by_id: HashMap<i32, usize>,
}

impl FairylandPuzzleTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<FairylandPuzzle> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&FairylandPuzzle> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[FairylandPuzzle] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FairylandPuzzle> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
