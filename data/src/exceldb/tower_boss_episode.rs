// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerBossEpisode {
    #[serde(rename = "bossLevel")]
    pub boss_level: i32,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "firstReward")]
    pub first_reward: String,
    #[serde(rename = "layerId")]
    pub layer_id: i32,
    #[serde(rename = "openRound")]
    pub open_round: i32,
    #[serde(rename = "preLayerId")]
    pub pre_layer_id: i32,
    #[serde(rename = "towerId")]
    pub tower_id: i32,
}

pub struct TowerBossEpisodeTable {
    records: Vec<TowerBossEpisode>,
}

impl TowerBossEpisodeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<TowerBossEpisode> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[TowerBossEpisode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerBossEpisode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
