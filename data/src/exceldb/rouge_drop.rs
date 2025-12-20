// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RougeDrop {
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(rename = "drop")]
    pub drop: String,
    #[serde(rename = "dropCount")]
    pub drop_count: String,
    #[serde(rename = "enterBag")]
    pub enter_bag: i32,
    #[serde(rename = "exp")]
    pub exp: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "notOwned")]
    pub not_owned: i32,
    #[serde(rename = "power")]
    pub power: i32,
    #[serde(rename = "selectCount")]
    pub select_count: i32,
    #[serde(rename = "talent")]
    pub talent: i32,
}

pub struct RougeDropTable {
    records: Vec<RougeDrop>,
    by_id: HashMap<i32, usize>,
}

impl RougeDropTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RougeDrop> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&RougeDrop> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[RougeDrop] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RougeDrop> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
