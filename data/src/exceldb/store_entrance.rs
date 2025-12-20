// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreEntrance {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "belongFirstTab")]
    pub belong_first_tab: i32,
    #[serde(rename = "belongSecondTab")]
    pub belong_second_tab: i32,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "openHideId")]
    pub open_hide_id: i32,
    #[serde(rename = "openId")]
    pub open_id: i32,
    #[serde(rename = "openTime")]
    pub open_time: String,
    #[serde(rename = "order")]
    pub order: i32,
    #[serde(rename = "prefab")]
    pub prefab: i32,
    #[serde(rename = "showCost")]
    pub show_cost: String,
    #[serde(rename = "storeId")]
    pub store_id: i32,
}

pub struct StoreEntranceTable {
    records: Vec<StoreEntrance>,
    by_id: HashMap<i32, usize>,
}

impl StoreEntranceTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<StoreEntrance> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&StoreEntrance> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[StoreEntrance] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, StoreEntrance> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
