// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExploreDialogue {
    #[serde(rename = "acceptButton")]
    pub accept_button: String,
    #[serde(rename = "audio")]
    pub audio: i32,
    #[serde(rename = "bonusButton")]
    pub bonus_button: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "interrupt")]
    pub interrupt: i32,
    #[serde(rename = "picture")]
    pub picture: String,
    #[serde(rename = "refuseButton")]
    pub refuse_button: String,
    #[serde(rename = "selectButton")]
    pub select_button: String,
    #[serde(rename = "speaker")]
    pub speaker: String,
    #[serde(rename = "stepid")]
    pub stepid: i32,
}

pub struct ExploreDialogueTable {
    records: Vec<ExploreDialogue>,
    by_id: HashMap<i32, usize>,
}

impl ExploreDialogueTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<ExploreDialogue> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&ExploreDialogue> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[ExploreDialogue] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, ExploreDialogue> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
