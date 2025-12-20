// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicCircle {
    #[serde(rename = "closeAniName")]
    pub close_ani_name: String,
    #[serde(rename = "closeAudio")]
    pub close_audio: i32,
    #[serde(rename = "closeEffect")]
    pub close_effect: String,
    #[serde(rename = "closeTime")]
    pub close_time: i32,
    #[serde(rename = "consumeNum")]
    pub consume_num: String,
    #[serde(rename = "consumeType")]
    pub consume_type: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "endSkills")]
    pub end_skills: String,
    #[serde(rename = "enemyAttrs")]
    pub enemy_attrs: String,
    #[serde(rename = "enemyBuff")]
    pub enemy_buff: String,
    #[serde(rename = "enemySkills")]
    pub enemy_skills: String,
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
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "posArr")]
    pub pos_arr: Option<serde_json::Value>,
    #[serde(rename = "round")]
    pub round: i32,
    #[serde(rename = "selfAttrs")]
    pub self_attrs: String,
    #[serde(rename = "selfBuff")]
    pub self_buff: String,
    #[serde(rename = "selfSkills")]
    pub self_skills: String,
    #[serde(rename = "uiType")]
    pub ui_type: i32,
}

pub struct MagicCircleTable {
    records: Vec<MagicCircle>,
    by_id: HashMap<i32, usize>,
}

impl MagicCircleTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<MagicCircle> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&MagicCircle> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[MagicCircle] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, MagicCircle> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
