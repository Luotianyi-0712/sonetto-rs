// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CritterTrainEvent {
    #[serde(rename = "addAttribute")]
    pub add_attribute: i32,
    #[serde(rename = "autoFinish")]
    pub auto_finish: bool,
    #[serde(rename = "computeIncrRate")]
    pub compute_incr_rate: bool,
    #[serde(rename = "condition")]
    pub condition: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "cost")]
    pub cost: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "effectAttribute")]
    pub effect_attribute: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "initStoryId")]
    pub init_story_id: i32,
    #[serde(rename = "maxCount")]
    pub max_count: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "normalStoryId")]
    pub normal_story_id: i32,
    #[serde(rename = "preferenceAttribute")]
    pub preference_attribute: String,
    #[serde(rename = "roomDialogId")]
    pub room_dialog_id: i32,
    #[serde(rename = "skilledStoryId")]
    pub skilled_story_id: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct CritterTrainEventTable {
    records: Vec<CritterTrainEvent>,
    by_id: HashMap<i32, usize>,
}

impl CritterTrainEventTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<CritterTrainEvent> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&CritterTrainEvent> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[CritterTrainEvent] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CritterTrainEvent> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
