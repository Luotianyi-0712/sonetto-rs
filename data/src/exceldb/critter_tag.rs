// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CritterTag {
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "effects")]
    pub effects: String,
    #[serde(rename = "filterTag")]
    pub filter_tag: String,
    #[serde(rename = "group")]
    pub group: i32,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "inheritance")]
    pub inheritance: bool,
    #[serde(rename = "luckyItemIds")]
    pub lucky_item_ids: String,
    #[serde(rename = "luckyItemType")]
    pub lucky_item_type: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "needAttributeLevel")]
    pub need_attribute_level: i32,
    #[serde(rename = "skillIcon")]
    pub skill_icon: String,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct CritterTagTable {
    records: Vec<CritterTag>,
    by_id: HashMap<String, usize>,
}

impl CritterTagTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<CritterTag> = if let Some(array) = value.as_array() {
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
            by_id.insert(record.id.clone(), idx);
        }
        
        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: String) -> Option<&CritterTag> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[CritterTag] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CritterTag> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
