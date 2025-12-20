// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightSkinReplaceMagicEffect {
    #[serde(rename = "closeAniName")]
    pub close_ani_name: String,
    #[serde(rename = "closeAudio")]
    pub close_audio: i32,
    #[serde(rename = "closeEffect")]
    pub close_effect: String,
    #[serde(rename = "closeTime")]
    pub close_time: i32,
    #[serde(rename = "enterAudio")]
    pub enter_audio: i32,
    #[serde(rename = "enterEffect")]
    pub enter_effect: String,
    #[serde(rename = "enterTime")]
    pub enter_time: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "loopEffect")]
    pub loop_effect: String,
    #[serde(rename = "posArr")]
    pub pos_arr: Vec<f32>,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
}

pub struct FightSkinReplaceMagicEffectTable {
    records: Vec<FightSkinReplaceMagicEffect>,
    by_id: HashMap<i32, usize>,
}

impl FightSkinReplaceMagicEffectTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<FightSkinReplaceMagicEffect> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&FightSkinReplaceMagicEffect> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[FightSkinReplaceMagicEffect] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightSkinReplaceMagicEffect> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
