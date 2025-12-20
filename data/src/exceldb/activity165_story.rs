// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity165Story {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "firstStepId")]
    pub first_step_id: i32,
    #[serde(rename = "firstUnlockElementCd1")]
    pub first_unlock_element_cd1: i32,
    #[serde(rename = "firstUnlockElementCd2")]
    pub first_unlock_element_cd2: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "pic")]
    pub pic: String,
    #[serde(rename = "preElementId1")]
    pub pre_element_id1: i32,
    #[serde(rename = "preElementId2")]
    pub pre_element_id2: i32,
    #[serde(rename = "storyId")]
    pub story_id: i32,
    #[serde(rename = "unlockElementIds1")]
    pub unlock_element_ids1: String,
    #[serde(rename = "unlockElementIds2")]
    pub unlock_element_ids2: String,
}

pub struct Activity165StoryTable {
    records: Vec<Activity165Story>,
}

impl Activity165StoryTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity165Story> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity165Story] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity165Story> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
