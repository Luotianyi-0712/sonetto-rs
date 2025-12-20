// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekwalkVer2Scene {
    #[serde(rename = "battleName")]
    pub battle_name: String,
    #[serde(rename = "buffId")]
    pub buff_id: i32,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "map")]
    pub map: String,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "name_en")]
    pub name_en: String,
    #[serde(rename = "typeName")]
    pub type_name: String,
}

pub struct WeekwalkVer2SceneTable {
    records: Vec<WeekwalkVer2Scene>,
    by_id: HashMap<i32, usize>,
}

impl WeekwalkVer2SceneTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<WeekwalkVer2Scene> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&WeekwalkVer2Scene> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[WeekwalkVer2Scene] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, WeekwalkVer2Scene> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
