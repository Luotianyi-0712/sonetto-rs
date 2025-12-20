// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoldierChess {
    #[serde(rename = "cost")]
    pub cost: String,
    #[serde(rename = "defaultPower")]
    pub default_power: i32,
    #[serde(rename = "formationDisplays")]
    pub formation_displays: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "resModel")]
    pub res_model: String,
    #[serde(rename = "resPic")]
    pub res_pic: String,
    #[serde(rename = "resZoom")]
    pub res_zoom: f32,
    #[serde(rename = "sell")]
    pub sell: i32,
    #[serde(rename = "skillId")]
    pub skill_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

pub struct SoldierChessTable {
    records: Vec<SoldierChess>,
    by_id: HashMap<i32, usize>,
}

impl SoldierChessTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SoldierChess> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SoldierChess> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SoldierChess] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SoldierChess> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
