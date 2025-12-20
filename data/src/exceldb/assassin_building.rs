// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssassinBuilding {
    #[serde(rename = "buildingBgIcon")]
    pub building_bg_icon: String,
    #[serde(rename = "buildingIcon")]
    pub building_icon: String,
    #[serde(rename = "cost")]
    pub cost: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "effectDesc")]
    pub effect_desc: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "itemIcon")]
    pub item_icon: String,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "levelupPic")]
    pub levelup_pic: String,
    #[serde(rename = "lockBuildingIcon")]
    pub lock_building_icon: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "unlockDesc")]
    pub unlock_desc: String,
}

pub struct AssassinBuildingTable {
    records: Vec<AssassinBuilding>,
    by_id: HashMap<i32, usize>,
}

impl AssassinBuildingTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<AssassinBuilding> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&AssassinBuilding> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[AssassinBuilding] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, AssassinBuilding> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
