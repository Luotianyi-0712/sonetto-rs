// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoChessEpisode {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "enemyId")]
    pub enemy_id: i32,
    #[serde(rename = "firstBounds")]
    pub first_bounds: String,
    #[serde(rename = "functionOn")]
    pub function_on: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "lostCondition")]
    pub lost_condition: String,
    #[serde(rename = "mallIds")]
    pub mall_ids: String,
    #[serde(rename = "masterId")]
    pub master_id: i32,
    #[serde(rename = "masterLibraryId")]
    pub master_library_id: i32,
    #[serde(rename = "maxRound")]
    pub max_round: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "npcEnemyId")]
    pub npc_enemy_id: i32,
    #[serde(rename = "preEpisode")]
    pub pre_episode: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "winCondition")]
    pub win_condition: String,
}

pub struct AutoChessEpisodeTable {
    records: Vec<AutoChessEpisode>,
    by_id: HashMap<i32, usize>,
}

impl AutoChessEpisodeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<AutoChessEpisode> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&AutoChessEpisode> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[AutoChessEpisode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, AutoChessEpisode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
