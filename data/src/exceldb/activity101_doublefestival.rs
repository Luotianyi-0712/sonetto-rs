// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity101Doublefestival {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "bgSpriteName")]
    pub bg_sprite_name: String,
    #[serde(rename = "blessContent")]
    pub bless_content: String,
    #[serde(rename = "blessContentType")]
    pub bless_content_type: String,
    #[serde(rename = "blessDesc")]
    pub bless_desc: String,
    #[serde(rename = "blessSpriteName")]
    pub bless_sprite_name: String,
    #[serde(rename = "blessTitle")]
    pub bless_title: String,
    #[serde(rename = "blessTitleEn")]
    pub bless_title_en: String,
    #[serde(rename = "btnDesc")]
    pub btn_desc: String,
    #[serde(rename = "day")]
    pub day: i32,
}

pub struct Activity101DoublefestivalTable {
    records: Vec<Activity101Doublefestival>,
}

impl Activity101DoublefestivalTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity101Doublefestival> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity101Doublefestival] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity101Doublefestival> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
