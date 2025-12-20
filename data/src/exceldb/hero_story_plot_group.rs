// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroStoryPlotGroup {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isEnd")]
    pub is_end: String,
    #[serde(rename = "place")]
    pub place: String,
    #[serde(rename = "roleName")]
    pub role_name: String,
    #[serde(rename = "storyId")]
    pub story_id: i32,
    #[serde(rename = "storyName")]
    pub story_name: String,
    #[serde(rename = "storyNameEn")]
    pub story_name_en: String,
    #[serde(rename = "storyPic")]
    pub story_pic: String,
    #[serde(rename = "time")]
    pub time: f32,
    #[serde(rename = "weather")]
    pub weather: i32,
}

pub struct HeroStoryPlotGroupTable {
    records: Vec<HeroStoryPlotGroup>,
    by_id: HashMap<i32, usize>,
}

impl HeroStoryPlotGroupTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<HeroStoryPlotGroup> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&HeroStoryPlotGroup> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[HeroStoryPlotGroup] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, HeroStoryPlotGroup> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
