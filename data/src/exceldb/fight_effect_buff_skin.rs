// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightEffectBuffSkin {
    #[serde(rename = "audio")]
    pub audio: i32,
    #[serde(rename = "buffId")]
    pub buff_id: i32,
    #[serde(rename = "delAudio")]
    pub del_audio: i32,
    #[serde(rename = "delEffect")]
    pub del_effect: String,
    #[serde(rename = "effectHang")]
    pub effect_hang: String,
    #[serde(rename = "effectPath")]
    pub effect_path: String,
    #[serde(rename = "orEnemy")]
    pub or_enemy: i32,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
    #[serde(rename = "triggerAudio")]
    pub trigger_audio: i32,
    #[serde(rename = "triggerEffect")]
    pub trigger_effect: String,
}

pub struct FightEffectBuffSkinTable {
    records: Vec<FightEffectBuffSkin>,
}

impl FightEffectBuffSkinTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<FightEffectBuffSkin> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[FightEffectBuffSkin] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightEffectBuffSkin> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
