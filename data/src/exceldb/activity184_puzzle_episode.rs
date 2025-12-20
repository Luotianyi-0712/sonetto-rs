// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity184PuzzleEpisode {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "illustrationCount")]
    pub illustration_count: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "staticShape")]
    pub static_shape: String,
    #[serde(rename = "target")]
    pub target: String,
    #[serde(rename = "titile")]
    pub titile: String,
    #[serde(rename = "titleTxt")]
    pub title_txt: String,
    #[serde(rename = "txt")]
    pub txt: String,
}

pub struct Activity184PuzzleEpisodeTable {
    records: Vec<Activity184PuzzleEpisode>,
    by_id: HashMap<i32, usize>,
}

impl Activity184PuzzleEpisodeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity184PuzzleEpisode> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity184PuzzleEpisode> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity184PuzzleEpisode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity184PuzzleEpisode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
