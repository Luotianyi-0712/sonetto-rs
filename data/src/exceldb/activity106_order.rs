// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity106Order {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "bonus")]
    pub bonus: String,
    #[serde(rename = "bossPic")]
    pub boss_pic: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "gameSetting")]
    pub game_setting: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "infoDesc")]
    pub info_desc: String,
    #[serde(rename = "jumpId")]
    pub jump_id: i32,
    #[serde(rename = "listenerParam")]
    pub listener_param: String,
    #[serde(rename = "listenerType")]
    pub listener_type: String,
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "maxProgress")]
    pub max_progress: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "openDay")]
    pub open_day: i32,
    #[serde(rename = "order")]
    pub order: i32,
    #[serde(rename = "rare")]
    pub rare: i32,
    #[serde(rename = "titledesc")]
    pub titledesc: String,
}

pub struct Activity106OrderTable {
    records: Vec<Activity106Order>,
    by_id: HashMap<i32, usize>,
}

impl Activity106OrderTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity106Order> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity106Order> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity106Order] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity106Order> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
