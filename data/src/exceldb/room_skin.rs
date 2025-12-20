// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomSkin {
    #[serde(rename = "activity")]
    pub activity: i32,
    #[serde(rename = "bannerIcon")]
    pub banner_icon: String,
    #[serde(rename = "buildId")]
    pub build_id: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "equipEffPos")]
    pub equip_eff_pos: Option<serde_json::Value>,
    #[serde(rename = "equipEffSize")]
    pub equip_eff_size: f32,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "priority")]
    pub priority: String,
    #[serde(rename = "rare")]
    pub rare: String,
    #[serde(rename = "sources")]
    pub sources: String,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct RoomSkinTable {
    records: Vec<RoomSkin>,
    by_id: HashMap<i32, usize>,
}

impl RoomSkinTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RoomSkin> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&RoomSkin> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[RoomSkin] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomSkin> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
