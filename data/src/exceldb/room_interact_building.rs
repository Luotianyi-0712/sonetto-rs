// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomInteractBuilding {
    #[serde(rename = "buildingAnim")]
    pub building_anim: String,
    #[serde(rename = "buildingId")]
    pub building_id: i32,
    #[serde(rename = "cameraId")]
    pub camera_id: i32,
    #[serde(rename = "heroAnimStr")]
    pub hero_anim_str: String,
    #[serde(rename = "heroCount")]
    pub hero_count: i32,
    #[serde(rename = "interactType")]
    pub interact_type: i32,
    #[serde(rename = "intervalTime")]
    pub interval_time: i32,
    #[serde(rename = "showTime")]
    pub show_time: i32,
}

pub struct RoomInteractBuildingTable {
    records: Vec<RoomInteractBuilding>,
}

impl RoomInteractBuildingTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RoomInteractBuilding> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[RoomInteractBuilding] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomInteractBuilding> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
