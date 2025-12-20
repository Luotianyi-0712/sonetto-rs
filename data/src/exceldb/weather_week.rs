// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherWeek {
    #[serde(rename = "day1")]
    pub day1: String,
    #[serde(rename = "day2")]
    pub day2: String,
    #[serde(rename = "day3")]
    pub day3: String,
    #[serde(rename = "day4")]
    pub day4: String,
    #[serde(rename = "day5")]
    pub day5: String,
    #[serde(rename = "day6")]
    pub day6: String,
    #[serde(rename = "day7")]
    pub day7: String,
    #[serde(rename = "id")]
    pub id: i32,
}

pub struct WeatherWeekTable {
    records: Vec<WeatherWeek>,
    by_id: HashMap<i32, usize>,
}

impl WeatherWeekTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<WeatherWeek> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&WeatherWeek> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[WeatherWeek] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, WeatherWeek> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
