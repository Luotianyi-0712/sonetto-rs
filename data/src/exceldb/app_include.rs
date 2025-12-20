// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInclude {
    #[serde(rename = "chapter")]
    pub chapter: Option<serde_json::Value>,
    #[serde(rename = "character")]
    pub character: Vec<i32>,
    #[serde(rename = "guide")]
    pub guide: Vec<serde_json::Value>,
    #[serde(rename = "heroStoryIds")]
    pub hero_story_ids: Vec<serde_json::Value>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "maxWeekWalk")]
    pub max_week_walk: i32,
    #[serde(rename = "path")]
    pub path: Option<serde_json::Value>,
    #[serde(rename = "roomTheme")]
    pub room_theme: Option<serde_json::Value>,
    #[serde(rename = "seasonIds")]
    pub season_ids: Vec<serde_json::Value>,
    #[serde(rename = "story")]
    pub story: Option<serde_json::Value>,
    #[serde(rename = "video")]
    pub video: Option<serde_json::Value>,
}

pub struct AppIncludeTable {
    records: Vec<AppInclude>,
    by_id: HashMap<i32, usize>,
}

impl AppIncludeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<AppInclude> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&AppInclude> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[AppInclude] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, AppInclude> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
