// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity142Episode {
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
    #[serde(rename = "lockSprite")]
    pub lock_sprite: String,
    #[serde(rename = "mainConditionStr")]
    pub main_condition_str: String,
    #[serde(rename = "mainConfition")]
    pub main_confition: String,
    #[serde(rename = "mapIds")]
    pub map_ids: String,
    #[serde(rename = "maxRound")]
    pub max_round: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "normalSprite")]
    pub normal_sprite: String,
    #[serde(rename = "openDay")]
    pub open_day: i32,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "pickUpObjIds")]
    pub pick_up_obj_ids: String,
    #[serde(rename = "preEpisode")]
    pub pre_episode: i32,
    #[serde(rename = "storyBefore")]
    pub story_before: i32,
    #[serde(rename = "storyClear")]
    pub story_clear: i32,
    #[serde(rename = "storyRepeat")]
    pub story_repeat: i32,
}

pub struct Activity142EpisodeTable {
    records: Vec<Activity142Episode>,
    by_id: HashMap<i32, usize>,
}

impl Activity142EpisodeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity142Episode> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity142Episode> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity142Episode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity142Episode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
