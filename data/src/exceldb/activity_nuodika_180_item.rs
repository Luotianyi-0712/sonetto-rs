// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityNuodika180Item {
    #[serde(rename = "canEmpty")]
    pub can_empty: i32,
    #[serde(rename = "canMove")]
    pub can_move: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "picture")]
    pub picture: String,
    #[serde(rename = "pictureOffset")]
    pub picture_offset: f32,
    #[serde(rename = "skillID")]
    pub skill_i_d: i32,
}

pub struct ActivityNuodika180ItemTable {
    records: Vec<ActivityNuodika180Item>,
}

impl ActivityNuodika180ItemTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<ActivityNuodika180Item> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[ActivityNuodika180Item] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, ActivityNuodika180Item> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
