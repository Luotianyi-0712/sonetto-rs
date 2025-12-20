// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiceCard {
    #[serde(rename = "aim1")]
    pub aim1: i32,
    #[serde(rename = "aim2")]
    pub aim2: i32,
    #[serde(rename = "aim3")]
    pub aim3: i32,
    #[serde(rename = "allRoundLimitCount")]
    pub all_round_limit_count: i32,
    #[serde(rename = "bufflist")]
    pub bufflist: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "effect1")]
    pub effect1: i32,
    #[serde(rename = "effect2")]
    pub effect2: i32,
    #[serde(rename = "effect3")]
    pub effect3: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "params1")]
    pub params1: String,
    #[serde(rename = "params2")]
    pub params2: String,
    #[serde(rename = "params3")]
    pub params3: String,
    #[serde(rename = "patternlist")]
    pub patternlist: String,
    #[serde(rename = "powerExtendRule")]
    pub power_extend_rule: i32,
    #[serde(rename = "quality")]
    pub quality: String,
    #[serde(rename = "roundLimitCount")]
    pub round_limit_count: i32,
    #[serde(rename = "spiritskilltype")]
    pub spiritskilltype: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct DiceCardTable {
    records: Vec<DiceCard>,
    by_id: HashMap<i32, usize>,
}

impl DiceCardTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<DiceCard> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&DiceCard> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[DiceCard] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, DiceCard> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
