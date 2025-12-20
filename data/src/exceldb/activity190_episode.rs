// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity190Episode {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "eliminateLevelId")]
    pub eliminate_level_id: i32,
    #[serde(rename = "enemyId")]
    pub enemy_id: String,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "gameId")]
    pub game_id: i32,
    #[serde(rename = "masterId")]
    pub master_id: i32,
    #[serde(rename = "maxRound")]
    pub max_round: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "preEpisodeBranchId")]
    pub pre_episode_branch_id: i32,
    #[serde(rename = "preEpisodeId")]
    pub pre_episode_id: i32,
    #[serde(rename = "storyBefore")]
    pub story_before: i32,
    #[serde(rename = "storyClear")]
    pub story_clear: i32,
    #[serde(rename = "type")]
    pub r#type: String,
}

pub struct Activity190EpisodeTable {
    records: Vec<Activity190Episode>,
}

impl Activity190EpisodeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity190Episode> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity190Episode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity190Episode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
