// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalItem {
    #[serde(rename = "copyItem")]
    pub copy_item: i32,
    #[serde(rename = "cost")]
    pub cost: String,
    #[serde(rename = "deposit")]
    pub deposit: i32,
    #[serde(rename = "desc1")]
    pub desc1: String,
    #[serde(rename = "desc2")]
    pub desc2: String,
    #[serde(rename = "disposable")]
    pub disposable: i32,
    #[serde(rename = "effect")]
    pub effect: String,
    #[serde(rename = "enhanceCond")]
    pub enhance_cond: String,
    #[serde(rename = "exchange")]
    pub exchange: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isDestroy")]
    pub is_destroy: i32,
    #[serde(rename = "mass")]
    pub mass: i32,
    #[serde(rename = "maxLimit")]
    pub max_limit: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "overlayLimit")]
    pub overlay_limit: i32,
    #[serde(rename = "rare")]
    pub rare: i32,
    #[serde(rename = "seasons")]
    pub seasons: String,
    #[serde(rename = "sellPrice")]
    pub sell_price: String,
    #[serde(rename = "subType")]
    pub sub_type: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "versions")]
    pub versions: String,
    #[serde(rename = "worth")]
    pub worth: i32,
}

pub struct SurvivalItemTable {
    records: Vec<SurvivalItem>,
    by_id: HashMap<i32, usize>,
}

impl SurvivalItemTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SurvivalItem> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SurvivalItem> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SurvivalItem] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SurvivalItem> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
