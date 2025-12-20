// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomCharacterAnim {
    #[serde(rename = "animName")]
    pub anim_name: String,
    #[serde(rename = "cameraState")]
    pub camera_state: i32,
    #[serde(rename = "downDuration")]
    pub down_duration: i32,
    #[serde(rename = "downTime")]
    pub down_time: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "upDistance")]
    pub up_distance: i32,
    #[serde(rename = "upDuration")]
    pub up_duration: i32,
    #[serde(rename = "upTime")]
    pub up_time: i32,
}

pub struct RoomCharacterAnimTable {
    records: Vec<RoomCharacterAnim>,
    by_id: HashMap<i32, usize>,
}

impl RoomCharacterAnimTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RoomCharacterAnim> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&RoomCharacterAnim> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[RoomCharacterAnim] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomCharacterAnim> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
