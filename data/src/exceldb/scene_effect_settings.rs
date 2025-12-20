// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneEffectSettings {
    #[serde(rename = "colorKey")]
    pub color_key: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "lightColor1")]
    pub light_color1: Vec<i32>,
    #[serde(rename = "lightColor2")]
    pub light_color2: Vec<i32>,
    #[serde(rename = "lightColor3")]
    pub light_color3: Vec<i32>,
    #[serde(rename = "lightColor4")]
    pub light_color4: Vec<i32>,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "sceneId")]
    pub scene_id: i32,
    #[serde(rename = "tag")]
    pub tag: String,
}

pub struct SceneEffectSettingsTable {
    records: Vec<SceneEffectSettings>,
    by_id: HashMap<i32, usize>,
}

impl SceneEffectSettingsTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SceneEffectSettings> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SceneEffectSettings> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SceneEffectSettings] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SceneEffectSettings> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
