// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssassinStealthMapGrid {
    #[serde(rename = "easyExplore")]
    pub easy_explore: bool,
    #[serde(rename = "gridId")]
    pub grid_id: i32,
    #[serde(rename = "gridImg")]
    pub grid_img: String,
    #[serde(rename = "gridParam")]
    pub grid_param: String,
    #[serde(rename = "gridType")]
    pub grid_type: i32,
    #[serde(rename = "hasFog")]
    pub has_fog: bool,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "point")]
    pub point: String,
    #[serde(rename = "pointShow")]
    pub point_show: String,
    #[serde(rename = "rotation")]
    pub rotation: i32,
    #[serde(rename = "x")]
    pub x: i32,
    #[serde(rename = "y")]
    pub y: i32,
}

pub struct AssassinStealthMapGridTable {
    records: Vec<AssassinStealthMapGrid>,
}

impl AssassinStealthMapGridTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<AssassinStealthMapGrid> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[AssassinStealthMapGrid] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, AssassinStealthMapGrid> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
