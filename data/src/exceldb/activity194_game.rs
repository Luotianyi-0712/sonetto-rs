// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity194Game {
    #[serde(rename = "eventGroup")]
    pub event_group: i32,
    #[serde(rename = "extraCondition")]
    pub extra_condition: String,
    #[serde(rename = "extraConditionStr")]
    pub extra_condition_str: String,
    #[serde(rename = "gameId")]
    pub game_id: i32,
    #[serde(rename = "initialItem")]
    pub initial_item: String,
    #[serde(rename = "initialTeam")]
    pub initial_team: String,
    #[serde(rename = "maxRoundLimit")]
    pub max_round_limit: i32,
    #[serde(rename = "startEnergy")]
    pub start_energy: i32,
    #[serde(rename = "winCondition")]
    pub win_condition: String,
    #[serde(rename = "winConditionStr")]
    pub win_condition_str: String,
}

pub struct Activity194GameTable {
    records: Vec<Activity194Game>,
}

impl Activity194GameTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity194Game> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity194Game] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity194Game> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
