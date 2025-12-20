// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RogueDifficulty {
    #[serde(rename = "attrAdd")]
    pub attr_add: String,
    #[serde(rename = "difficulty")]
    pub difficulty: i32,
    #[serde(rename = "effect1")]
    pub effect1: String,
    #[serde(rename = "effect2")]
    pub effect2: String,
    #[serde(rename = "effect3")]
    pub effect3: String,
    #[serde(rename = "effectDesc1")]
    pub effect_desc1: String,
    #[serde(rename = "effectDesc2")]
    pub effect_desc2: String,
    #[serde(rename = "effectDesc3")]
    pub effect_desc3: String,
    #[serde(rename = "initCurrency")]
    pub init_currency: String,
    #[serde(rename = "initHeart")]
    pub init_heart: i32,
    #[serde(rename = "initHeroCount")]
    pub init_hero_count: i32,
    #[serde(rename = "initLevel")]
    pub init_level: String,
    #[serde(rename = "initRoom")]
    pub init_room: i32,
    #[serde(rename = "preDifficulty")]
    pub pre_difficulty: i32,
    #[serde(rename = "retries")]
    pub retries: i32,
    #[serde(rename = "rule")]
    pub rule: String,
    #[serde(rename = "scoreAdd")]
    pub score_add: i32,
    #[serde(rename = "showDifficulty")]
    pub show_difficulty: i32,
    #[serde(rename = "title")]
    pub title: String,
}

pub struct RogueDifficultyTable {
    records: Vec<RogueDifficulty>,
}

impl RogueDifficultyTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RogueDifficulty> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[RogueDifficulty] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RogueDifficulty> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
