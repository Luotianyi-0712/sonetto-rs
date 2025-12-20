// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreDecorate {
    #[serde(rename = "biglmg")]
    pub biglmg: String,
    #[serde(rename = "buylmg")]
    pub buylmg: String,
    #[serde(rename = "decorateskinOffset")]
    pub decorateskin_offset: String,
    #[serde(rename = "decorateskinl2dOffset")]
    pub decorateskinl2d_offset: String,
    #[serde(rename = "effectbiglmg")]
    pub effectbiglmg: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "maxbuycountType")]
    pub maxbuycount_type: i32,
    #[serde(rename = "offTag")]
    pub off_tag: i32,
    #[serde(rename = "onlineTag")]
    pub online_tag: i32,
    #[serde(rename = "originalCost1")]
    pub original_cost1: i32,
    #[serde(rename = "originalCost2")]
    pub original_cost2: i32,
    #[serde(rename = "productType")]
    pub product_type: i32,
    #[serde(rename = "rare")]
    pub rare: i32,
    #[serde(rename = "showskinId")]
    pub showskin_id: i32,
    #[serde(rename = "smalllmg")]
    pub smalllmg: String,
    #[serde(rename = "storeld")]
    pub storeld: i32,
    #[serde(rename = "subType")]
    pub sub_type: i32,
    #[serde(rename = "tag1")]
    pub tag1: String,
    #[serde(rename = "typeName")]
    pub type_name: String,
    #[serde(rename = "video")]
    pub video: String,
}

pub struct StoreDecorateTable {
    records: Vec<StoreDecorate>,
    by_id: HashMap<i32, usize>,
}

impl StoreDecorateTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<StoreDecorate> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&StoreDecorate> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[StoreDecorate] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, StoreDecorate> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
