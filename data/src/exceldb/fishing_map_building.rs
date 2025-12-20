// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FishingMapBuilding {
    #[serde(rename = "defineId")]
    pub define_id: i32,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "resAreaDirection")]
    pub res_area_direction: i32,
    #[serde(rename = "rotate")]
    pub rotate: i32,
    #[serde(rename = "uid")]
    pub uid: i32,
    #[serde(rename = "use")]
    pub r#use: bool,
    #[serde(rename = "x")]
    pub x: i32,
    #[serde(rename = "y")]
    pub y: i32,
}

pub struct FishingMapBuildingTable {
    records: Vec<FishingMapBuilding>,
}

impl FishingMapBuildingTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<FishingMapBuilding> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[FishingMapBuilding] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FishingMapBuilding> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
