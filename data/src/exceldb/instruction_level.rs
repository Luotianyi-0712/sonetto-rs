// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionLevel {
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "desc_en")]
    pub desc_en: String,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "instructionDesc")]
    pub instruction_desc: String,
    #[serde(rename = "picRes")]
    pub pic_res: String,
    #[serde(rename = "preEpisode")]
    pub pre_episode: i32,
    #[serde(rename = "topicId")]
    pub topic_id: i32,
}

pub struct InstructionLevelTable {
    records: Vec<InstructionLevel>,
    by_id: HashMap<i32, usize>,
}

impl InstructionLevelTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<InstructionLevel> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&InstructionLevel> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[InstructionLevel] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, InstructionLevel> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
