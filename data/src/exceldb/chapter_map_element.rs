// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterMapElement {
    #[serde(rename = "acceptText")]
    pub accept_text: String,
    #[serde(rename = "condition")]
    pub condition: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "dispatchingText")]
    pub dispatching_text: String,
    #[serde(rename = "effect")]
    pub effect: String,
    #[serde(rename = "finishText")]
    pub finish_text: String,
    #[serde(rename = "flagText")]
    pub flag_text: String,
    #[serde(rename = "fragment")]
    pub fragment: i32,
    #[serde(rename = "holeSize")]
    pub hole_size: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "offsetPos")]
    pub offset_pos: String,
    #[serde(rename = "param")]
    pub param: String,
    #[serde(rename = "paramCn")]
    pub param_cn: String,
    #[serde(rename = "paramLang")]
    pub param_lang: String,
    #[serde(rename = "permanentReward")]
    pub permanent_reward: String,
    #[serde(rename = "pos")]
    pub pos: String,
    #[serde(rename = "res")]
    pub res: String,
    #[serde(rename = "resScale")]
    pub res_scale: f32,
    #[serde(rename = "retroReward")]
    pub retro_reward: String,
    #[serde(rename = "reward")]
    pub reward: String,
    #[serde(rename = "rewardPoint")]
    pub reward_point: i32,
    #[serde(rename = "showArrow")]
    pub show_arrow: i32,
    #[serde(rename = "showCamera")]
    pub show_camera: i32,
    #[serde(rename = "tipOffsetPos")]
    pub tip_offset_pos: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct ChapterMapElementTable {
    records: Vec<ChapterMapElement>,
    by_id: HashMap<i32, usize>,
}

impl ChapterMapElementTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<ChapterMapElement> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&ChapterMapElement> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[ChapterMapElement] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, ChapterMapElement> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
