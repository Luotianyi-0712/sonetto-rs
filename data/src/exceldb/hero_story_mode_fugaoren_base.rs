// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroStoryModeFugaorenBase {
    #[serde(rename = "areaId")]
    pub area_id: i32,
    #[serde(rename = "choose")]
    pub choose: String,
    #[serde(rename = "conArea")]
    pub con_area: String,
    #[serde(rename = "costTime")]
    pub cost_time: String,
    #[serde(rename = "dialogId")]
    pub dialog_id: String,
    #[serde(rename = "endTime")]
    pub end_time: f32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "preId")]
    pub pre_id: i32,
    #[serde(rename = "resource")]
    pub resource: String,
    #[serde(rename = "rightChoose")]
    pub right_choose: i32,
    #[serde(rename = "startTime")]
    pub start_time: f32,
    #[serde(rename = "storyId")]
    pub story_id: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "unlockAreaId")]
    pub unlock_area_id: i32,
    #[serde(rename = "weather")]
    pub weather: i32,
}

pub struct HeroStoryModeFugaorenBaseTable {
    records: Vec<HeroStoryModeFugaorenBase>,
    by_id: HashMap<i32, usize>,
}

impl HeroStoryModeFugaorenBaseTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<HeroStoryModeFugaorenBase> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&HeroStoryModeFugaorenBase> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[HeroStoryModeFugaorenBase] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, HeroStoryModeFugaorenBase> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
