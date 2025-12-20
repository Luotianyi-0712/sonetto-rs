// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomCharacterInteraction {
    #[serde(rename = "behaviour")]
    pub behaviour: i32,
    #[serde(rename = "buildingAnimState")]
    pub building_anim_state: String,
    #[serde(rename = "buildingAudio")]
    pub building_audio: i32,
    #[serde(rename = "buildingCameraIds")]
    pub building_camera_ids: String,
    #[serde(rename = "buildingId")]
    pub building_id: i32,
    #[serde(rename = "buildingInside")]
    pub building_inside: bool,
    #[serde(rename = "buildingInsideSpines")]
    pub building_inside_spines: String,
    #[serde(rename = "buildingNode")]
    pub building_node: String,
    #[serde(rename = "conditionStr")]
    pub condition_str: String,
    #[serde(rename = "delayEnterBuilding")]
    pub delay_enter_building: i32,
    #[serde(rename = "dialogId")]
    pub dialog_id: i32,
    #[serde(rename = "excludeDaily")]
    pub exclude_daily: bool,
    #[serde(rename = "faithDialog")]
    pub faith_dialog: i32,
    #[serde(rename = "heroAnimState")]
    pub hero_anim_state: String,
    #[serde(rename = "heroId")]
    pub hero_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "rate")]
    pub rate: i32,
    #[serde(rename = "relateHeroId")]
    pub relate_hero_id: i32,
    #[serde(rename = "reward")]
    pub reward: String,
    #[serde(rename = "showtime")]
    pub showtime: i32,
    #[serde(rename = "variety")]
    pub variety: i32,
    #[serde(rename = "weather")]
    pub weather: i32,
}

pub struct RoomCharacterInteractionTable {
    records: Vec<RoomCharacterInteraction>,
    by_id: HashMap<i32, usize>,
}

impl RoomCharacterInteractionTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RoomCharacterInteraction> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&RoomCharacterInteraction> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[RoomCharacterInteraction] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomCharacterInteraction> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
