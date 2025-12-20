// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity192Map {
    #[serde(rename = "colliderHeight")]
    pub collider_height: f32,
    #[serde(rename = "colliderOffsetX")]
    pub collider_offset_x: f32,
    #[serde(rename = "colliderOffsetY")]
    pub collider_offset_y: f32,
    #[serde(rename = "colliderWidth")]
    pub collider_width: f32,
    #[serde(rename = "componentId")]
    pub component_id: i32,
    #[serde(rename = "componentType")]
    pub component_type: i32,
    #[serde(rename = "extraParams")]
    pub extra_params: String,
    #[serde(rename = "height")]
    pub height: f32,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "posX")]
    pub pos_x: f32,
    #[serde(rename = "posY")]
    pub pos_y: f32,
    #[serde(rename = "rotation")]
    pub rotation: f32,
    #[serde(rename = "scale")]
    pub scale: f32,
    #[serde(rename = "width")]
    pub width: f32,
}

pub struct Activity192MapTable {
    records: Vec<Activity192Map>,
}

impl Activity192MapTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity192Map> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity192Map] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity192Map> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
