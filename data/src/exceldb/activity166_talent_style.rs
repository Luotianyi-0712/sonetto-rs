// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity166TalentStyle {
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "needStar")]
    pub need_star: i32,
    #[serde(rename = "skillId")]
    pub skill_id: String,
    #[serde(rename = "skillId2")]
    pub skill_id2: String,
    #[serde(rename = "slot")]
    pub slot: i32,
    #[serde(rename = "talentId")]
    pub talent_id: i32,
}

pub struct Activity166TalentStyleTable {
    records: Vec<Activity166TalentStyle>,
}

impl Activity166TalentStyleTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity166TalentStyle> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity166TalentStyle] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity166TalentStyle> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
