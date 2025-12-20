// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity152 {
    #[serde(rename = "acceptDate")]
    pub accept_date: String,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "bonus")]
    pub bonus: String,
    #[serde(rename = "dialog")]
    pub dialog: String,
    #[serde(rename = "presentIcon")]
    pub present_icon: String,
    #[serde(rename = "presentId")]
    pub present_id: i32,
    #[serde(rename = "presentName")]
    pub present_name: String,
    #[serde(rename = "presentSign")]
    pub present_sign: String,
    #[serde(rename = "roleIcon")]
    pub role_icon: String,
    #[serde(rename = "roleName")]
    pub role_name: String,
    #[serde(rename = "roleNameEn")]
    pub role_name_en: String,
}

pub struct Activity152Table {
    records: Vec<Activity152>,
}

impl Activity152Table {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity152> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity152] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity152> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
