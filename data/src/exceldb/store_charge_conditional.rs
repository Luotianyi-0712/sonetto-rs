// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreChargeConditional {
    #[serde(rename = "bigImg2")]
    pub big_img2: String,
    #[serde(rename = "bigImg3")]
    pub big_img3: String,
    #[serde(rename = "bonus")]
    pub bonus: String,
    #[serde(rename = "conDesc")]
    pub con_desc: String,
    #[serde(rename = "goodsId")]
    pub goods_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "idsStr")]
    pub ids_str: String,
    #[serde(rename = "listenerParam")]
    pub listener_param: String,
    #[serde(rename = "listenerType")]
    pub listener_type: String,
    #[serde(rename = "maxProgress")]
    pub max_progress: i32,
    #[serde(rename = "order2")]
    pub order2: i32,
}

pub struct StoreChargeConditionalTable {
    records: Vec<StoreChargeConditional>,
    by_id: HashMap<i32, usize>,
}

impl StoreChargeConditionalTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<StoreChargeConditional> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&StoreChargeConditional> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[StoreChargeConditional] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, StoreChargeConditional> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
