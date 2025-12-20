// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightVoice {
    #[serde(rename = "audio_type1")]
    pub audio_type1: i32,
    #[serde(rename = "audio_type10")]
    pub audio_type10: i32,
    #[serde(rename = "audio_type2")]
    pub audio_type2: i32,
    #[serde(rename = "audio_type3")]
    pub audio_type3: i32,
    #[serde(rename = "audio_type4")]
    pub audio_type4: i32,
    #[serde(rename = "audio_type5")]
    pub audio_type5: i32,
    #[serde(rename = "audio_type6")]
    pub audio_type6: i32,
    #[serde(rename = "audio_type7")]
    pub audio_type7: i32,
    #[serde(rename = "audio_type8")]
    pub audio_type8: i32,
    #[serde(rename = "audio_type9")]
    pub audio_type9: i32,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
}

pub struct FightVoiceTable {
    records: Vec<FightVoice>,
}

impl FightVoiceTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<FightVoice> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[FightVoice] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightVoice> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
