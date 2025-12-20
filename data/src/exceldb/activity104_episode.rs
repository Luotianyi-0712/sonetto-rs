// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity104Episode {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "afterStoryId")]
    pub after_story_id: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "firstPassEquipId")]
    pub first_pass_equip_id: i32,
    #[serde(rename = "layer")]
    pub layer: i32,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "stage")]
    pub stage: i32,
    #[serde(rename = "stageName")]
    pub stage_name: String,
    #[serde(rename = "stageNameEn")]
    pub stage_name_en: String,
    #[serde(rename = "stagePicture")]
    pub stage_picture: String,
    #[serde(rename = "unlockEquipIndex")]
    pub unlock_equip_index: String,
}

pub struct Activity104EpisodeTable {
    records: Vec<Activity104Episode>,
}

impl Activity104EpisodeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity104Episode> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity104Episode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity104Episode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
