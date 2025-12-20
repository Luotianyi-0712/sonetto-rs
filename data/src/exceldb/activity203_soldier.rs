// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity203Soldier {
    #[serde(rename = "animation")]
    pub animation: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "hP")]
    pub h_p: i32,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "passiveSkill")]
    pub passive_skill: i32,
    #[serde(rename = "resource")]
    pub resource: String,
    #[serde(rename = "scale")]
    pub scale: f32,
    #[serde(rename = "soldierId")]
    pub soldier_id: i32,
    #[serde(rename = "speed")]
    pub speed: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct Activity203SoldierTable {
    records: Vec<Activity203Soldier>,
}

impl Activity203SoldierTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity203Soldier> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity203Soldier] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity203Soldier> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
