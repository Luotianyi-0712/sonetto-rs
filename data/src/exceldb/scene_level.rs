// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneLevel {
    #[serde(rename = "ambientSound")]
    pub ambient_sound: i32,
    #[serde(rename = "bgm")]
    pub bgm: i32,
    #[serde(rename = "bloomA")]
    pub bloom_a: f32,
    #[serde(rename = "bloomB")]
    pub bloom_b: f32,
    #[serde(rename = "bloomEffect")]
    pub bloom_effect: String,
    #[serde(rename = "bloomG")]
    pub bloom_g: f32,
    #[serde(rename = "bloomR")]
    pub bloom_r: f32,
    #[serde(rename = "cameraId")]
    pub camera_id: i32,
    #[serde(rename = "cameraOffset")]
    pub camera_offset: String,
    #[serde(rename = "cardCamera")]
    pub card_camera: String,
    #[serde(rename = "flickerSceneFactor")]
    pub flicker_scene_factor: f32,
    #[serde(rename = "flyEffect")]
    pub fly_effect: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "resName")]
    pub res_name: String,
    #[serde(rename = "sceneEffects")]
    pub scene_effects: String,
    #[serde(rename = "sceneId")]
    pub scene_id: i32,
    #[serde(rename = "sceneType")]
    pub scene_type: i32,
    #[serde(rename = "spineB")]
    pub spine_b: f32,
    #[serde(rename = "spineG")]
    pub spine_g: f32,
    #[serde(rename = "spineR")]
    pub spine_r: f32,
    #[serde(rename = "useBloom")]
    pub use_bloom: i32,
    #[serde(rename = "wadeEffect")]
    pub wade_effect: String,
    #[serde(rename = "weatherEffect")]
    pub weather_effect: i32,
}

pub struct SceneLevelTable {
    records: Vec<SceneLevel>,
    by_id: HashMap<i32, usize>,
}

impl SceneLevelTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SceneLevel> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SceneLevel> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SceneLevel] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SceneLevel> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
