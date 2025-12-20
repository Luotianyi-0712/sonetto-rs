// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAttribute {
    #[serde(rename = "attrType")]
    pub attr_type: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isShow")]
    pub is_show: i32,
    #[serde(rename = "isShowTips")]
    pub is_show_tips: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "showType")]
    pub show_type: i32,
    #[serde(rename = "showcolor")]
    pub showcolor: i32,
    #[serde(rename = "sortId")]
    pub sort_id: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct CharacterAttributeTable {
    records: Vec<CharacterAttribute>,
    by_id: HashMap<i32, usize>,
}

impl CharacterAttributeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<CharacterAttribute> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&CharacterAttribute> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[CharacterAttribute] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CharacterAttribute> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
