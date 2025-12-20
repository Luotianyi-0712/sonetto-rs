// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity174TestBot {
    #[serde(rename = "collection1")]
    pub collection1: i32,
    #[serde(rename = "collection2")]
    pub collection2: i32,
    #[serde(rename = "collection3")]
    pub collection3: i32,
    #[serde(rename = "collection4")]
    pub collection4: i32,
    #[serde(rename = "costCoin")]
    pub cost_coin: i32,
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "enhance")]
    pub enhance: String,
    #[serde(rename = "robotId")]
    pub robot_id: i32,
    #[serde(rename = "role1")]
    pub role1: i32,
    #[serde(rename = "role2")]
    pub role2: i32,
    #[serde(rename = "role3")]
    pub role3: i32,
    #[serde(rename = "role4")]
    pub role4: i32,
}

pub struct Activity174TestBotTable {
    records: Vec<Activity174TestBot>,
}

impl Activity174TestBotTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity174TestBot> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity174TestBot] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity174TestBot> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
