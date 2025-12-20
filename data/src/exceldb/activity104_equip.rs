// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity104Equip {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "attrId")]
    pub attr_id: i32,
    #[serde(rename = "career")]
    pub career: String,
    #[serde(rename = "careerIcon")]
    pub career_icon: String,
    #[serde(rename = "equipId")]
    pub equip_id: i32,
    #[serde(rename = "group")]
    pub group: i32,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "iconOffset")]
    pub icon_offset: String,
    #[serde(rename = "isOptional")]
    pub is_optional: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "rare")]
    pub rare: i32,
    #[serde(rename = "signIcon")]
    pub sign_icon: String,
    #[serde(rename = "signOffset")]
    pub sign_offset: String,
    #[serde(rename = "skillId")]
    pub skill_id: String,
    #[serde(rename = "tag")]
    pub tag: String,
}

pub struct Activity104EquipTable {
    records: Vec<Activity104Equip>,
}

impl Activity104EquipTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity104Equip> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity104Equip] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity104Equip> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
