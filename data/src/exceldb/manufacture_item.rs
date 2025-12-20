// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufactureItem {
    #[serde(rename = "batchIcon")]
    pub batch_icon: String,
    #[serde(rename = "batchName")]
    pub batch_name: String,
    #[serde(rename = "criProductionCount")]
    pub cri_production_count: i32,
    #[serde(rename = "criProductionId")]
    pub cri_production_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "needMat")]
    pub need_mat: String,
    #[serde(rename = "needTime")]
    pub need_time: i32,
    #[serde(rename = "orderPrice")]
    pub order_price: i32,
    #[serde(rename = "showInAdvancedOrder")]
    pub show_in_advanced_order: bool,
    #[serde(rename = "unitCount")]
    pub unit_count: i32,
    #[serde(rename = "wholesalePrice")]
    pub wholesale_price: i32,
}

pub struct ManufactureItemTable {
    records: Vec<ManufactureItem>,
    by_id: HashMap<i32, usize>,
}

impl ManufactureItemTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<ManufactureItem> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&ManufactureItem> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[ManufactureItem] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, ManufactureItem> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
