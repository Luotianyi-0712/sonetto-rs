// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandbookStoryGroup {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "fragmentIdList")]
    pub fragment_id_list: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "levelIdDict")]
    pub level_id_dict: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "order")]
    pub order: i32,
    #[serde(rename = "storyChapterId")]
    pub story_chapter_id: i32,
    #[serde(rename = "storyIdList")]
    pub story_id_list: String,
    #[serde(rename = "time")]
    pub time: String,
    #[serde(rename = "year")]
    pub year: String,
}

pub struct HandbookStoryGroupTable {
    records: Vec<HandbookStoryGroup>,
    by_id: HashMap<i32, usize>,
}

impl HandbookStoryGroupTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<HandbookStoryGroup> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&HandbookStoryGroup> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[HandbookStoryGroup] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, HandbookStoryGroup> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
