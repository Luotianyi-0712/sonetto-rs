// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity122Character {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "characterType")]
    pub character_type: i32,
    #[serde(rename = "destroyObstacle")]
    pub destroy_obstacle: i32,
    #[serde(rename = "fireDecrHp")]
    pub fire_decr_hp: i32,
    #[serde(rename = "moveObstacle")]
    pub move_obstacle: i32,
    #[serde(rename = "pushOverObstacle")]
    pub push_over_obstacle: i32,
    #[serde(rename = "trapDecrHp")]
    pub trap_decr_hp: i32,
}

pub struct Activity122CharacterTable {
    records: Vec<Activity122Character>,
}

impl Activity122CharacterTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity122Character> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity122Character] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity122Character> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
