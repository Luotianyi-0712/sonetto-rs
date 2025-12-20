// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoChessEnemyFormation {
    #[serde(rename = "enemyId")]
    pub enemy_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "index1")]
    pub index1: String,
    #[serde(rename = "index1Buff")]
    pub index1_buff: String,
    #[serde(rename = "index2")]
    pub index2: String,
    #[serde(rename = "index2Buff")]
    pub index2_buff: String,
    #[serde(rename = "index3")]
    pub index3: String,
    #[serde(rename = "index3Buff")]
    pub index3_buff: String,
    #[serde(rename = "index4")]
    pub index4: String,
    #[serde(rename = "index4Buff")]
    pub index4_buff: String,
    #[serde(rename = "index5")]
    pub index5: String,
    #[serde(rename = "index5Buff")]
    pub index5_buff: String,
    #[serde(rename = "round")]
    pub round: i32,
    #[serde(rename = "zoneId")]
    pub zone_id: i32,
}

pub struct AutoChessEnemyFormationTable {
    records: Vec<AutoChessEnemyFormation>,
    by_id: HashMap<i32, usize>,
}

impl AutoChessEnemyFormationTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<AutoChessEnemyFormation> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&AutoChessEnemyFormation> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[AutoChessEnemyFormation] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, AutoChessEnemyFormation> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
