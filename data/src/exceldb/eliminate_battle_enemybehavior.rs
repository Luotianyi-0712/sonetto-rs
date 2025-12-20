// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EliminateBattleEnemybehavior {
    #[serde(rename = "behavior")]
    pub behavior: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "list1")]
    pub list1: String,
    #[serde(rename = "list2")]
    pub list2: String,
    #[serde(rename = "list3")]
    pub list3: String,
    #[serde(rename = "prob1")]
    pub prob1: String,
    #[serde(rename = "prob2")]
    pub prob2: String,
    #[serde(rename = "prob3")]
    pub prob3: String,
    #[serde(rename = "round")]
    pub round: i32,
}

pub struct EliminateBattleEnemybehaviorTable {
    records: Vec<EliminateBattleEnemybehavior>,
    by_id: HashMap<i32, usize>,
}

impl EliminateBattleEnemybehaviorTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<EliminateBattleEnemybehavior> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&EliminateBattleEnemybehavior> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[EliminateBattleEnemybehavior] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, EliminateBattleEnemybehavior> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
