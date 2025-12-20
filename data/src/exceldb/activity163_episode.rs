// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity163Episode {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "afterStoryId")]
    pub after_story_id: i32,
    #[serde(rename = "beforeStoryId")]
    pub before_story_id: i32,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "evidenceId")]
    pub evidence_id: String,
    #[serde(rename = "failedId")]
    pub failed_id: i32,
    #[serde(rename = "initialCluesIds")]
    pub initial_clues_ids: String,
    #[serde(rename = "initialDialog")]
    pub initial_dialog: i32,
    #[serde(rename = "maxError")]
    pub max_error: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "opponentPieces")]
    pub opponent_pieces: String,
    #[serde(rename = "playerPieces")]
    pub player_pieces: String,
    #[serde(rename = "preEpisodeId")]
    pub pre_episode_id: i32,
    #[serde(rename = "promptsNum")]
    pub prompts_num: i32,
}

pub struct Activity163EpisodeTable {
    records: Vec<Activity163Episode>,
}

impl Activity163EpisodeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity163Episode> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity163Episode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity163Episode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
