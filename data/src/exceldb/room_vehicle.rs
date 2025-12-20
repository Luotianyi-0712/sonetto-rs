// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomVehicle {
    #[serde(rename = "audioCrossload")]
    pub audio_crossload: i32,
    #[serde(rename = "audioStop")]
    pub audio_stop: i32,
    #[serde(rename = "audioTurn")]
    pub audio_turn: i32,
    #[serde(rename = "audioTurnAround")]
    pub audio_turn_around: i32,
    #[serde(rename = "audioWalk")]
    pub audio_walk: i32,
    #[serde(rename = "buildIcon")]
    pub build_icon: String,
    #[serde(rename = "endPathWaitTime")]
    pub end_path_wait_time: i32,
    #[serde(rename = "firstCameraId")]
    pub first_camera_id: i32,
    #[serde(rename = "followNodePathStr")]
    pub follow_node_path_str: String,
    #[serde(rename = "followRadiusStr")]
    pub follow_radius_str: String,
    #[serde(rename = "followRotateStr")]
    pub follow_rotate_str: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "moveSpeed")]
    pub move_speed: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "radius")]
    pub radius: i32,
    #[serde(rename = "replaceConditionStr")]
    pub replace_condition_str: String,
    #[serde(rename = "resId")]
    pub res_id: i32,
    #[serde(rename = "rotate")]
    pub rotate: i32,
    #[serde(rename = "rotationSpeed")]
    pub rotation_speed: i32,
    #[serde(rename = "takeoffTime")]
    pub takeoff_time: i32,
    #[serde(rename = "thirdCameraId")]
    pub third_camera_id: i32,
    #[serde(rename = "uiIcon")]
    pub ui_icon: String,
    #[serde(rename = "useType")]
    pub use_type: i32,
    #[serde(rename = "waterOffseY")]
    pub water_offse_y: i32,
}

pub struct RoomVehicleTable {
    records: Vec<RoomVehicle>,
    by_id: HashMap<i32, usize>,
}

impl RoomVehicleTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RoomVehicle> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&RoomVehicle> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[RoomVehicle] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomVehicle> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
