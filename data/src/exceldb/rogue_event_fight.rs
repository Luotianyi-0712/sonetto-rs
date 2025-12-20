// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RogueEventFight {
    #[serde(rename = "attrChange1")]
    pub attr_change1: String,
    #[serde(rename = "attrChange2")]
    pub attr_change2: String,
    #[serde(rename = "attrChange3")]
    pub attr_change3: String,
    #[serde(rename = "attrChange4")]
    pub attr_change4: String,
    #[serde(rename = "attrChange5")]
    pub attr_change5: String,
    #[serde(rename = "episode")]
    pub episode: i32,
    #[serde(rename = "heartChange1")]
    pub heart_change1: String,
    #[serde(rename = "heartChange2")]
    pub heart_change2: String,
    #[serde(rename = "heartChange3")]
    pub heart_change3: String,
    #[serde(rename = "heartChange4")]
    pub heart_change4: String,
    #[serde(rename = "heartChange5")]
    pub heart_change5: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isChangeScene")]
    pub is_change_scene: i32,
    #[serde(rename = "newtemplate")]
    pub newtemplate: String,
    #[serde(rename = "oldtemplate")]
    pub oldtemplate: String,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct RogueEventFightTable {
    records: Vec<RogueEventFight>,
    by_id: HashMap<i32, usize>,
}

impl RogueEventFightTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RogueEventFight> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&RogueEventFight> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[RogueEventFight] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RogueEventFight> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
