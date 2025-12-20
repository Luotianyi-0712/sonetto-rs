// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity128Assess {
    #[serde(rename = "battleIconBg")]
    pub battle_icon_bg: String,
    #[serde(rename = "layer4Assess")]
    pub layer4_assess: i32,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "mainBg")]
    pub main_bg: String,
    #[serde(rename = "needPointBoss1")]
    pub need_point_boss1: i32,
    #[serde(rename = "needPointBoss2")]
    pub need_point_boss2: i32,
    #[serde(rename = "needPointBoss3")]
    pub need_point_boss3: i32,
    #[serde(rename = "spriteName")]
    pub sprite_name: String,
    #[serde(rename = "strLevel")]
    pub str_level: String,
}

pub struct Activity128AssessTable {
    records: Vec<Activity128Assess>,
}

impl Activity128AssessTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity128Assess> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity128Assess] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity128Assess> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
