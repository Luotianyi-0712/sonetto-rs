// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity174Bot {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "collection1")]
    pub collection1: i32,
    #[serde(rename = "collection2")]
    pub collection2: i32,
    #[serde(rename = "collection3")]
    pub collection3: i32,
    #[serde(rename = "collection4")]
    pub collection4: i32,
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "enhance")]
    pub enhance: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "level")]
    pub level: i32,
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
    #[serde(rename = "season")]
    pub season: i32,
}

pub struct Activity174BotTable {
    records: Vec<Activity174Bot>,
    by_id: HashMap<i32, usize>,
}

impl Activity174BotTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity174Bot> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity174Bot> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity174Bot] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity174Bot> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
