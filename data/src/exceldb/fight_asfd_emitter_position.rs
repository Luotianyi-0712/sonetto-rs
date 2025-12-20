// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightAsfdEmitterPosition {
    #[serde(rename = "emitterId")]
    pub emitter_id: i32,
    #[serde(rename = "enemySidePos")]
    pub enemy_side_pos: Vec<f32>,
    #[serde(rename = "mySidePos")]
    pub my_side_pos: Vec<f32>,
    #[serde(rename = "sceneId")]
    pub scene_id: i32,
}

pub struct FightAsfdEmitterPositionTable {
    records: Vec<FightAsfdEmitterPosition>,
}

impl FightAsfdEmitterPositionTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<FightAsfdEmitterPosition> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[FightAsfdEmitterPosition] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightAsfdEmitterPosition> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
