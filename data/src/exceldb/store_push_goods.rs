// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorePushGoods {
    #[serde(rename = "className")]
    pub class_name: i32,
    #[serde(rename = "containedgoodsId")]
    pub containedgoods_id: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "gapHours")]
    pub gap_hours: i32,
    #[serde(rename = "goodpushsId")]
    pub goodpushs_id: i32,
    #[serde(rename = "levelLimits")]
    pub level_limits: String,
    #[serde(rename = "listenerParam")]
    pub listener_param: String,
    #[serde(rename = "listenerType")]
    pub listener_type: String,
    #[serde(rename = "typeId")]
    pub type_id: i32,
}

pub struct StorePushGoodsTable {
    records: Vec<StorePushGoods>,
}

impl StorePushGoodsTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<StorePushGoods> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[StorePushGoods] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, StorePushGoods> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
