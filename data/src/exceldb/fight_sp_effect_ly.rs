// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightSpEffectLy {
    #[serde(rename = "audioId")]
    pub audio_id: i32,
    #[serde(rename = "fadeAudioId")]
    pub fade_audio_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "pos")]
    pub pos: Vec<f32>,
    #[serde(rename = "spine1EffectRes")]
    pub spine1_effect_res: String,
    #[serde(rename = "spine1Res")]
    pub spine1_res: String,
    #[serde(rename = "spine2EffectRes")]
    pub spine2_effect_res: String,
    #[serde(rename = "spine2Res")]
    pub spine2_res: String,
}

pub struct FightSpEffectLyTable {
    records: Vec<FightSpEffectLy>,
    by_id: HashMap<i32, usize>,
}

impl FightSpEffectLyTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<FightSpEffectLy> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&FightSpEffectLy> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[FightSpEffectLy] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightSpEffectLy> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
