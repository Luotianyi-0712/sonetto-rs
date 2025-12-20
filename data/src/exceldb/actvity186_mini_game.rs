// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actvity186MiniGame {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "conditionStage")]
    pub condition_stage: String,
    #[serde(rename = "expireSeconds")]
    pub expire_seconds: i32,
    #[serde(rename = "gameType2Prob")]
    pub game_type2_prob: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "triggerConditionParams")]
    pub trigger_condition_params: String,
    #[serde(rename = "triggerConditionType")]
    pub trigger_condition_type: i32,
}

pub struct Actvity186MiniGameTable {
    records: Vec<Actvity186MiniGame>,
    by_id: HashMap<i32, usize>,
}

impl Actvity186MiniGameTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Actvity186MiniGame> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Actvity186MiniGame> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Actvity186MiniGame] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Actvity186MiniGame> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
