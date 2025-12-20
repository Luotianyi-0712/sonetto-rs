// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomCharacter {
    #[serde(rename = "cameraAnimPath")]
    pub camera_anim_path: String,
    #[serde(rename = "effectPath")]
    pub effect_path: String,
    #[serde(rename = "hideFootprint")]
    pub hide_footprint: i32,
    #[serde(rename = "moveInterval")]
    pub move_interval: i32,
    #[serde(rename = "moveRate")]
    pub move_rate: i32,
    #[serde(rename = "moveSpeed")]
    pub move_speed: f32,
    #[serde(rename = "roleVoice")]
    pub role_voice: String,
    #[serde(rename = "shadow")]
    pub shadow: String,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
    #[serde(rename = "specialIdle")]
    pub special_idle: i32,
    #[serde(rename = "specialRate")]
    pub special_rate: i32,
    #[serde(rename = "waterDistance")]
    pub water_distance: i32,
    #[serde(rename = "zeroMix")]
    pub zero_mix: bool,
}

pub struct RoomCharacterTable {
    records: Vec<RoomCharacter>,
}

impl RoomCharacterTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RoomCharacter> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[RoomCharacter] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomCharacter> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
