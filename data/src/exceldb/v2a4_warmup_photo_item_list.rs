// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V2a4WarmupPhotoItemList {
    #[serde(rename = "failTalk")]
    pub fail_talk: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "imgName")]
    pub img_name: String,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "no1")]
    pub no1: i32,
    #[serde(rename = "no2")]
    pub no2: i32,
    #[serde(rename = "no3")]
    pub no3: i32,
    #[serde(rename = "passTalk")]
    pub pass_talk: i32,
    #[serde(rename = "passTalkAllYes")]
    pub pass_talk_all_yes: i32,
    #[serde(rename = "preTalk")]
    pub pre_talk: i32,
    #[serde(rename = "yes1")]
    pub yes1: i32,
    #[serde(rename = "yes2")]
    pub yes2: i32,
    #[serde(rename = "yes3")]
    pub yes3: i32,
}

pub struct V2a4WarmupPhotoItemListTable {
    records: Vec<V2a4WarmupPhotoItemList>,
    by_id: HashMap<i32, usize>,
}

impl V2a4WarmupPhotoItemListTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<V2a4WarmupPhotoItemList> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&V2a4WarmupPhotoItemList> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[V2a4WarmupPhotoItemList] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, V2a4WarmupPhotoItemList> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
