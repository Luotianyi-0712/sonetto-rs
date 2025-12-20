// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalShelterIntrudeFight {
    #[serde(rename = "battleId")]
    pub battle_id: i32,
    #[serde(rename = "cleanpoint")]
    pub cleanpoint: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "destructionLevel")]
    pub destruction_level: String,
    #[serde(rename = "drop")]
    pub drop: i32,
    #[serde(rename = "gridType")]
    pub grid_type: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "saveMonster")]
    pub save_monster: String,
    #[serde(rename = "scale")]
    pub scale: i32,
    #[serde(rename = "score")]
    pub score: i32,
    #[serde(rename = "smallheadicon")]
    pub smallheadicon: String,
    #[serde(rename = "target")]
    pub target: String,
    #[serde(rename = "toward")]
    pub toward: String,
}

pub struct SurvivalShelterIntrudeFightTable {
    records: Vec<SurvivalShelterIntrudeFight>,
    by_id: HashMap<i32, usize>,
}

impl SurvivalShelterIntrudeFightTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SurvivalShelterIntrudeFight> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SurvivalShelterIntrudeFight> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SurvivalShelterIntrudeFight] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SurvivalShelterIntrudeFight> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
