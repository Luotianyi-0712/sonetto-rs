// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity144Episode {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "bgPath")]
    pub bg_path: String,
    #[serde(rename = "bonus")]
    pub bonus: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nameen")]
    pub nameen: String,
    #[serde(rename = "picture")]
    pub picture: String,
    #[serde(rename = "preEpisode")]
    pub pre_episode: i32,
    #[serde(rename = "showBonus")]
    pub show_bonus: String,
    #[serde(rename = "showTargets")]
    pub show_targets: String,
    #[serde(rename = "storyBefore")]
    pub story_before: i32,
    #[serde(rename = "storyClear")]
    pub story_clear: i32,
    #[serde(rename = "taskId")]
    pub task_id: i32,
    #[serde(rename = "unlockDesc")]
    pub unlock_desc: String,
}

pub struct Activity144EpisodeTable {
    records: Vec<Activity144Episode>,
}

impl Activity144EpisodeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity144Episode> = if let Some(array) = value.as_array() {
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
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity144Episode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity144Episode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
