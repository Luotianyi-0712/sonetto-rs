// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    #[serde(rename = "actionList")]
    pub action_list: String,
    #[serde(rename = "battleId")]
    pub battle_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "limit")]
    pub limit: i32,
    #[serde(rename = "limitOneTurn")]
    pub limit_one_turn: i32,
    #[serde(rename = "param1")]
    pub param1: String,
    #[serde(rename = "param10")]
    pub param10: String,
    #[serde(rename = "param2")]
    pub param2: String,
    #[serde(rename = "param3")]
    pub param3: String,
    #[serde(rename = "param4")]
    pub param4: String,
    #[serde(rename = "param5")]
    pub param5: String,
    #[serde(rename = "param6")]
    pub param6: String,
    #[serde(rename = "param7")]
    pub param7: String,
    #[serde(rename = "param8")]
    pub param8: String,
    #[serde(rename = "param9")]
    pub param9: String,
    #[serde(rename = "triggerType")]
    pub trigger_type: String,
}

pub struct TriggerTable {
    records: Vec<Trigger>,
    by_id: HashMap<i32, usize>,
}

impl TriggerTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Trigger> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Trigger> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Trigger] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Trigger> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
