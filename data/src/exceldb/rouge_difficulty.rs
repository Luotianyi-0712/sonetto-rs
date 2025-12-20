// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RougeDifficulty {
    #[serde(rename = "balanceLevel")]
    pub balance_level: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "difficulty")]
    pub difficulty: i32,
    #[serde(rename = "initCollections")]
    pub init_collections: String,
    #[serde(rename = "initEffects")]
    pub init_effects: String,
    #[serde(rename = "monsterDesc")]
    pub monster_desc: String,
    #[serde(rename = "preDifficulty")]
    pub pre_difficulty: i32,
    #[serde(rename = "scoreReward")]
    pub score_reward: i32,
    #[serde(rename = "season")]
    pub season: i32,
    #[serde(rename = "startView")]
    pub start_view: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "title_en")]
    pub title_en: String,
    #[serde(rename = "version")]
    pub version: String,
}

pub struct RougeDifficultyTable {
    records: Vec<RougeDifficulty>,
}

impl RougeDifficultyTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RougeDifficulty> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[RougeDifficulty] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RougeDifficulty> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
