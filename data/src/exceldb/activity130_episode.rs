// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity130Episode {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "afterStoryId")]
    pub after_story_id: i32,
    #[serde(rename = "beforeStoryId")]
    pub before_story_id: i32,
    #[serde(rename = "conditionStr")]
    pub condition_str: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "elements")]
    pub elements: String,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "episodetag")]
    pub episodetag: String,
    #[serde(rename = "lvscene")]
    pub lvscene: i32,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "preEpisodeId")]
    pub pre_episode_id: i32,
    #[serde(rename = "scenes")]
    pub scenes: String,
    #[serde(rename = "winCondition")]
    pub win_condition: String,
}

pub struct Activity130EpisodeTable {
    records: Vec<Activity130Episode>,
}

impl Activity130EpisodeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity130Episode> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity130Episode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity130Episode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
