// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekwalkType {
    #[serde(rename = "canResetLayer")]
    pub can_reset_layer: i32,
    #[serde(rename = "heroCd")]
    pub hero_cd: i32,
    #[serde(rename = "isRefresh")]
    pub is_refresh: i32,
    #[serde(rename = "showDetail")]
    pub show_detail: i32,
    #[serde(rename = "star")]
    pub star: i32,
    #[serde(rename = "starNum")]
    pub star_num: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct WeekwalkTypeTable {
    records: Vec<WeekwalkType>,
}

impl WeekwalkTypeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<WeekwalkType> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[WeekwalkType] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, WeekwalkType> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
