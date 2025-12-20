// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightBuffLayerEffectEnemySkin {
    #[serde(rename = "addLayerAudio")]
    pub add_layer_audio: i32,
    #[serde(rename = "addLayerEffect")]
    pub add_layer_effect: String,
    #[serde(rename = "addLayerEffectRoot")]
    pub add_layer_effect_root: String,
    #[serde(rename = "createAudio")]
    pub create_audio: i32,
    #[serde(rename = "createEffect")]
    pub create_effect: String,
    #[serde(rename = "createEffectRoot")]
    pub create_effect_root: String,
    #[serde(rename = "delayTimeBeforeLoop")]
    pub delay_time_before_loop: i32,
    #[serde(rename = "destroyEffect")]
    pub destroy_effect: String,
    #[serde(rename = "destroyEffectAudio")]
    pub destroy_effect_audio: i32,
    #[serde(rename = "destroyEffectRoot")]
    pub destroy_effect_root: String,
    #[serde(rename = "hideWhenPlayTimeline")]
    pub hide_when_play_timeline: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "layer")]
    pub layer: i32,
    #[serde(rename = "loopEffect")]
    pub loop_effect: String,
    #[serde(rename = "loopEffectAudio")]
    pub loop_effect_audio: i32,
    #[serde(rename = "loopEffectRoot")]
    pub loop_effect_root: String,
    #[serde(rename = "releaseAddLayerEffectTime")]
    pub release_add_layer_effect_time: i32,
    #[serde(rename = "releaseCreateEffectTime")]
    pub release_create_effect_time: i32,
    #[serde(rename = "releaseDestroyEffectTime")]
    pub release_destroy_effect_time: i32,
    #[serde(rename = "skin")]
    pub skin: i32,
}

pub struct FightBuffLayerEffectEnemySkinTable {
    records: Vec<FightBuffLayerEffectEnemySkin>,
    by_id: HashMap<i32, usize>,
}

impl FightBuffLayerEffectEnemySkinTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<FightBuffLayerEffectEnemySkin> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&FightBuffLayerEffectEnemySkin> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[FightBuffLayerEffectEnemySkin] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightBuffLayerEffectEnemySkin> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
