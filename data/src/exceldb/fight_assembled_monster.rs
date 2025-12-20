// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightAssembledMonster {
    #[serde(rename = "clickIndex")]
    pub click_index: i32,
    #[serde(rename = "hpPos")]
    pub hp_pos: Option<serde_json::Value>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "part")]
    pub part: i32,
    #[serde(rename = "selectPos")]
    pub select_pos: Option<serde_json::Value>,
    #[serde(rename = "virtualSpinePos")]
    pub virtual_spine_pos: Vec<f32>,
    #[serde(rename = "virtualSpineSize")]
    pub virtual_spine_size: Option<serde_json::Value>,
    #[serde(rename = "virtualStance")]
    pub virtual_stance: Option<serde_json::Value>,
}

pub struct FightAssembledMonsterTable {
    records: Vec<FightAssembledMonster>,
    by_id: HashMap<i32, usize>,
}

impl FightAssembledMonsterTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<FightAssembledMonster> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&FightAssembledMonster> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[FightAssembledMonster] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightAssembledMonster> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
