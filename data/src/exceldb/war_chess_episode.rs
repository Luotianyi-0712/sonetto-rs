// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarChessEpisode {
    #[serde(rename = "chessScene")]
    pub chess_scene: String,
    #[serde(rename = "dialogueId")]
    pub dialogue_id: String,
    #[serde(rename = "eliminateScene")]
    pub eliminate_scene: String,
    #[serde(rename = "enemyId")]
    pub enemy_id: i32,
    #[serde(rename = "extraWinCondition")]
    pub extra_win_condition: String,
    #[serde(rename = "extraWinConditionDesc")]
    pub extra_win_condition_desc: String,
    #[serde(rename = "failCondition")]
    pub fail_condition: String,
    #[serde(rename = "failConditionDesc")]
    pub fail_condition_desc: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "maxRound")]
    pub max_round: i32,
    #[serde(rename = "preset")]
    pub preset: i32,
    #[serde(rename = "presetCharacter")]
    pub preset_character: i32,
    #[serde(rename = "presetSoldier")]
    pub preset_soldier: Option<serde_json::Value>,
    #[serde(rename = "strongHoldIds")]
    pub strong_hold_ids: String,
    #[serde(rename = "unlockCharacterIds")]
    pub unlock_character_ids: String,
    #[serde(rename = "unlockChessSoldierIds")]
    pub unlock_chess_soldier_ids: String,
    #[serde(rename = "unlockSlotIds")]
    pub unlock_slot_ids: String,
    #[serde(rename = "winCondition")]
    pub win_condition: String,
    #[serde(rename = "winConditionDesc")]
    pub win_condition_desc: String,
}

pub struct WarChessEpisodeTable {
    records: Vec<WarChessEpisode>,
    by_id: HashMap<i32, usize>,
}

impl WarChessEpisodeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<WarChessEpisode> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&WarChessEpisode> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[WarChessEpisode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, WarChessEpisode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
