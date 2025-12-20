// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroStory {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "activity_pic")]
    pub activity_pic: String,
    #[serde(rename = "bonus")]
    pub bonus: String,
    #[serde(rename = "cgBg")]
    pub cg_bg: String,
    #[serde(rename = "cgPos")]
    pub cg_pos: String,
    #[serde(rename = "cgScale")]
    pub cg_scale: String,
    #[serde(rename = "cgUnlockEpisodeId")]
    pub cg_unlock_episode_id: i32,
    #[serde(rename = "cgUnlockStoryId")]
    pub cg_unlock_story_id: i32,
    #[serde(rename = "challengeBonus")]
    pub challenge_bonus: String,
    #[serde(rename = "chapterId")]
    pub chapter_id: i32,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "heroName")]
    pub hero_name: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "main_pic")]
    pub main_pic: String,
    #[serde(rename = "mainviewName")]
    pub mainview_name: String,
    #[serde(rename = "monster_pic")]
    pub monster_pic: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "order")]
    pub order: i32,
    #[serde(rename = "permanentUnlock")]
    pub permanent_unlock: String,
    #[serde(rename = "photo")]
    pub photo: String,
    #[serde(rename = "queryVersion")]
    pub query_version: String,
    #[serde(rename = "signature")]
    pub signature: String,
    #[serde(rename = "unlock")]
    pub unlock: String,
}

pub struct HeroStoryTable {
    records: Vec<HeroStory>,
    by_id: HashMap<i32, usize>,
}

impl HeroStoryTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<HeroStory> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&HeroStory> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[HeroStory] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, HeroStory> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
