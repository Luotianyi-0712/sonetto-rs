// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity144Equip {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "buffId")]
    pub buff_id: i32,
    #[serde(rename = "cost")]
    pub cost: String,
    #[serde(rename = "effectDesc")]
    pub effect_desc: String,
    #[serde(rename = "equipId")]
    pub equip_id: i32,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "preEquipId")]
    pub pre_equip_id: i32,
    #[serde(rename = "typeId")]
    pub type_id: i32,
}

pub struct Activity144EquipTable {
    records: Vec<Activity144Equip>,
}

impl Activity144EquipTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity144Equip> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity144Equip] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity144Equip> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
