// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomCameraParams {
    #[serde(rename = "angle")]
    pub angle: String,
    #[serde(rename = "bendingAmount")]
    pub bending_amount: String,
    #[serde(rename = "blur")]
    pub blur: String,
    #[serde(rename = "distance")]
    pub distance: String,
    #[serde(rename = "fogFarColorRGBA")]
    pub fog_far_color_r_g_b_a: String,
    #[serde(rename = "fogNearColorRGBA")]
    pub fog_near_color_r_g_b_a: String,
    #[serde(rename = "fogParticles")]
    pub fog_particles: String,
    #[serde(rename = "fogRangeXYZW")]
    pub fog_range_x_y_z_w: String,
    #[serde(rename = "fogViewType")]
    pub fog_view_type: String,
    #[serde(rename = "gameMode")]
    pub game_mode: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "lightMin")]
    pub light_min: String,
    #[serde(rename = "oceanFog")]
    pub ocean_fog: String,
    #[serde(rename = "offsetHorizon")]
    pub offset_horizon: String,
    #[serde(rename = "shadowOffsetXYZW")]
    pub shadow_offset_x_y_z_w: String,
    #[serde(rename = "state")]
    pub state: i32,
    #[serde(rename = "touchMoveSpeed")]
    pub touch_move_speed: String,
    #[serde(rename = "zoom")]
    pub zoom: i32,
}

pub struct RoomCameraParamsTable {
    records: Vec<RoomCameraParams>,
    by_id: HashMap<i32, usize>,
}

impl RoomCameraParamsTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RoomCameraParams> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&RoomCameraParams> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[RoomCameraParams] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomCameraParams> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
