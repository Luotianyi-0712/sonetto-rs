// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity109Episode {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "chapterId")]
    pub chapter_id: i32,
    #[serde(rename = "conditionStr")]
    pub condition_str: String,
    #[serde(rename = "extStarCondition")]
    pub ext_star_condition: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "maxRound")]
    pub max_round: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "openDay")]
    pub open_day: i32,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "preEpisode")]
    pub pre_episode: i32,
    #[serde(rename = "storyBefore")]
    pub story_before: i32,
    #[serde(rename = "storyClear")]
    pub story_clear: i32,
    #[serde(rename = "winCondition")]
    pub win_condition: String,
}

pub struct Activity109EpisodeTable {
    records: Vec<Activity109Episode>,
    by_id: HashMap<i32, usize>,
}

impl Activity109EpisodeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity109Episode> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity109Episode> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity109Episode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity109Episode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
