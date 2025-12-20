// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomSceneAmbient {
    #[serde(rename = "addRange")]
    pub add_range: f32,
    #[serde(rename = "ambientcol")]
    pub ambientcol: Vec<f32>,
    #[serde(rename = "ambientsize")]
    pub ambientsize: f32,
    #[serde(rename = "bendingAmount")]
    pub bending_amount: f32,
    #[serde(rename = "cameraDistanceValue")]
    pub camera_distance_value: f32,
    #[serde(rename = "dirLightColor")]
    pub dir_light_color: Vec<f32>,
    #[serde(rename = "dirLightIntensity")]
    pub dir_light_intensity: f32,
    #[serde(rename = "disfogedge")]
    pub disfogedge: f32,
    #[serde(rename = "disfogstart")]
    pub disfogstart: f32,
    #[serde(rename = "fogColor")]
    pub fog_color: Vec<f32>,
    #[serde(rename = "fogColor2")]
    pub fog_color2: Vec<f32>,
    #[serde(rename = "fogHeight")]
    pub fog_height: f32,
    #[serde(rename = "fogMainCol")]
    pub fog_main_col: Vec<f32>,
    #[serde(rename = "fogOutSideCol")]
    pub fog_out_side_col: Vec<f32>,
    #[serde(rename = "fogParams")]
    pub fog_params: Vec<f32>,
    #[serde(rename = "fogPlaneMainCol")]
    pub fog_plane_main_col: Vec<f32>,
    #[serde(rename = "fogPlaneOutSideCol")]
    pub fog_plane_out_side_col: Vec<f32>,
    #[serde(rename = "fogdensitymax")]
    pub fogdensitymax: f32,
    #[serde(rename = "fogdensitymin")]
    pub fogdensitymin: f32,
    #[serde(rename = "heightfogedge")]
    pub heightfogedge: f32,
    #[serde(rename = "hflambert")]
    pub hflambert: f32,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "insideShadow")]
    pub inside_shadow: f32,
    #[serde(rename = "lightDir")]
    pub light_dir: Vec<f32>,
    #[serde(rename = "lightOffset")]
    pub light_offset: f32,
    #[serde(rename = "lightOffsetFar")]
    pub light_offset_far: f32,
    #[serde(rename = "lightOffsetNear")]
    pub light_offset_near: f32,
    #[serde(rename = "lightParams")]
    pub light_params: Vec<f32>,
    #[serde(rename = "lightRange")]
    pub light_range: f32,
    #[serde(rename = "lightRangeFar")]
    pub light_range_far: f32,
    #[serde(rename = "lightRangeNear")]
    pub light_range_near: f32,
    #[serde(rename = "lightmax")]
    pub lightmax: f32,
    #[serde(rename = "lightmin")]
    pub lightmin: f32,
    #[serde(rename = "outsideShadow")]
    pub outside_shadow: f32,
    #[serde(rename = "rimcol")]
    pub rimcol: Vec<f32>,
    #[serde(rename = "shadowColor")]
    pub shadow_color: Vec<f32>,
}

pub struct RoomSceneAmbientTable {
    records: Vec<RoomSceneAmbient>,
    by_id: HashMap<String, usize>,
}

impl RoomSceneAmbientTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RoomSceneAmbient> = if let Some(array) = value.as_array() {
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
            by_id.insert(record.id.clone(), idx);
        }
        
        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: String) -> Option<&RoomSceneAmbient> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[RoomSceneAmbient] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomSceneAmbient> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
