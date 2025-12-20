// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RougeFightEvent {
    #[serde(rename = "advanceInteractive")]
    pub advance_interactive: String,
    #[serde(rename = "bossDesc")]
    pub boss_desc: String,
    #[serde(rename = "bossMask")]
    pub boss_mask: String,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "episodeIdInstead")]
    pub episode_id_instead: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "interactive")]
    pub interactive: String,
    #[serde(rename = "isChangeScene")]
    pub is_change_scene: i32,
    #[serde(rename = "monsterMask")]
    pub monster_mask: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "versionEpisode")]
    pub version_episode: String,
    #[serde(rename = "versionEvent")]
    pub version_event: String,
    #[serde(rename = "versionInteractive")]
    pub version_interactive: String,
}

pub struct RougeFightEventTable {
    records: Vec<RougeFightEvent>,
    by_id: HashMap<i32, usize>,
}

impl RougeFightEventTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RougeFightEvent> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&RougeFightEvent> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[RougeFightEvent] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RougeFightEvent> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
