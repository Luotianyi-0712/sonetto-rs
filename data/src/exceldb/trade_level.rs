// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeLevel {
    #[serde(rename = "addBlockMax")]
    pub add_block_max: i32,
    #[serde(rename = "bonus")]
    pub bonus: String,
    #[serde(rename = "dimension")]
    pub dimension: String,
    #[serde(rename = "job")]
    pub job: String,
    #[serde(rename = "jobCard")]
    pub job_card: String,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "levelUpNeedTask")]
    pub level_up_need_task: i32,
    #[serde(rename = "maxRestBuildingNum")]
    pub max_rest_building_num: i32,
    #[serde(rename = "maxTrainSlotCount")]
    pub max_train_slot_count: i32,
    #[serde(rename = "silenceBonus")]
    pub silence_bonus: String,
    #[serde(rename = "taskName")]
    pub task_name: String,
    #[serde(rename = "trainsRoundCount")]
    pub trains_round_count: i32,
    #[serde(rename = "unlockId")]
    pub unlock_id: String,
}

pub struct TradeLevelTable {
    records: Vec<TradeLevel>,
}

impl TradeLevelTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<TradeLevel> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[TradeLevel] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TradeLevel> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
