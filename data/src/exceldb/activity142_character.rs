// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity142Character {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "canBreakTile")]
    pub can_break_tile: i32,
    #[serde(rename = "canLightBrazier")]
    pub can_light_brazier: i32,
    #[serde(rename = "canTriggerPedal")]
    pub can_trigger_pedal: i32,
    #[serde(rename = "canUseFireball")]
    pub can_use_fireball: i32,
    #[serde(rename = "characterType")]
    pub character_type: i32,
}

pub struct Activity142CharacterTable {
    records: Vec<Activity142Character>,
}

impl Activity142CharacterTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity142Character> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity142Character] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity142Character> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
