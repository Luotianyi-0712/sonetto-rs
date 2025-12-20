// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterLimited {
    #[serde(rename = "actionTime")]
    pub action_time: Option<serde_json::Value>,
    #[serde(rename = "audio")]
    pub audio: i32,
    #[serde(rename = "effectDuration")]
    pub effect_duration: i32,
    #[serde(rename = "entranceEffect")]
    pub entrance_effect: String,
    #[serde(rename = "entranceMv")]
    pub entrance_mv: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mvtime")]
    pub mvtime: i32,
    #[serde(rename = "specialInsightDesc")]
    pub special_insight_desc: String,
    #[serde(rename = "specialLive2d")]
    pub special_live2d: String,
    #[serde(rename = "spine")]
    pub spine: String,
    #[serde(rename = "spineParam")]
    pub spine_param: Vec<i32>,
    #[serde(rename = "stopAudio")]
    pub stop_audio: i32,
    #[serde(rename = "voice")]
    pub voice: Option<serde_json::Value>,
}

pub struct CharacterLimitedTable {
    records: Vec<CharacterLimited>,
    by_id: HashMap<i32, usize>,
}

impl CharacterLimitedTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<CharacterLimited> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&CharacterLimited> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[CharacterLimited] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CharacterLimited> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
