// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity203Ai {
    #[serde(rename = "ai_index")]
    pub ai_index: i32,
    #[serde(rename = "assist_weight")]
    pub assist_weight: i32,
    #[serde(rename = "attack_trigger_rate")]
    pub attack_trigger_rate: i32,
    #[serde(rename = "attack_weight")]
    pub attack_weight: i32,
    #[serde(rename = "gaptime")]
    pub gaptime: i32,
    #[serde(rename = "hero_go_front_ornot")]
    pub hero_go_front_ornot: bool,
    #[serde(rename = "hero_move_rate")]
    pub hero_move_rate: i32,
    #[serde(rename = "hero_or_soldier")]
    pub hero_or_soldier: bool,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "negative_move_trigger_rate")]
    pub negative_move_trigger_rate: i32,
    #[serde(rename = "negative_move_weight")]
    pub negative_move_weight: i32,
    #[serde(rename = "positive_move_trigger_rate")]
    pub positive_move_trigger_rate: i32,
    #[serde(rename = "positive_move_weight")]
    pub positive_move_weight: i32,
}

pub struct Activity203AiTable {
    records: Vec<Activity203Ai>,
    by_id: HashMap<i32, usize>,
}

impl Activity203AiTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity203Ai> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity203Ai> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity203Ai] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity203Ai> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
