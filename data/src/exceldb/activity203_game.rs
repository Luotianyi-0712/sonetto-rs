// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity203Game {
    #[serde(rename = "battleGroup")]
    pub battle_group: i32,
    #[serde(rename = "battlePic")]
    pub battle_pic: String,
    #[serde(rename = "battleTime")]
    pub battle_time: i32,
    #[serde(rename = "battledesc")]
    pub battledesc: String,
    #[serde(rename = "gameId")]
    pub game_id: i32,
    #[serde(rename = "gameTarget")]
    pub game_target: String,
    #[serde(rename = "loseTarget")]
    pub lose_target: String,
    #[serde(rename = "skill")]
    pub skill: String,
    #[serde(rename = "targetDesc")]
    pub target_desc: String,
}

pub struct Activity203GameTable {
    records: Vec<Activity203Game>,
}

impl Activity203GameTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity203Game> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity203Game] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity203Game> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
