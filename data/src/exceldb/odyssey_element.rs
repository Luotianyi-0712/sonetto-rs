// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdysseyElement {
    #[serde(rename = "dataBase")]
    pub data_base: i32,
    #[serde(rename = "heroPos")]
    pub hero_pos: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "iconFrame")]
    pub icon_frame: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isPermanent")]
    pub is_permanent: i32,
    #[serde(rename = "main")]
    pub main: i32,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "needFollow")]
    pub need_follow: i32,
    #[serde(rename = "pos")]
    pub pos: String,
    #[serde(rename = "refreshType")]
    pub refresh_type: i32,
    #[serde(rename = "taskDesc")]
    pub task_desc: String,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "unlockCondition")]
    pub unlock_condition: String,
}

pub struct OdysseyElementTable {
    records: Vec<OdysseyElement>,
    by_id: HashMap<i32, usize>,
}

impl OdysseyElementTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<OdysseyElement> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&OdysseyElement> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[OdysseyElement] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, OdysseyElement> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
