// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity114Event {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "battleId")]
    pub battle_id: i32,
    #[serde(rename = "checkAttribute")]
    pub check_attribute: String,
    #[serde(rename = "checkOptionText")]
    pub check_option_text: String,
    #[serde(rename = "checkfeatures")]
    pub checkfeatures: String,
    #[serde(rename = "condition")]
    pub condition: String,
    #[serde(rename = "disposable")]
    pub disposable: i32,
    #[serde(rename = "eventType")]
    pub event_type: i32,
    #[serde(rename = "failureStoryId")]
    pub failure_story_id: i32,
    #[serde(rename = "failureVerify")]
    pub failure_verify: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isCheckEvent")]
    pub is_check_event: i32,
    #[serde(rename = "isTransition")]
    pub is_transition: String,
    #[serde(rename = "nonOptionText")]
    pub non_option_text: String,
    #[serde(rename = "param")]
    pub param: String,
    #[serde(rename = "storyId")]
    pub story_id: i32,
    #[serde(rename = "successBattle")]
    pub success_battle: String,
    #[serde(rename = "successStoryId")]
    pub success_story_id: String,
    #[serde(rename = "successVerify")]
    pub success_verify: String,
    #[serde(rename = "testId")]
    pub test_id: i32,
    #[serde(rename = "threshold")]
    pub threshold: i32,
}

pub struct Activity114EventTable {
    records: Vec<Activity114Event>,
    by_id: HashMap<i32, usize>,
}

impl Activity114EventTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity114Event> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity114Event> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity114Event] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity114Event> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
