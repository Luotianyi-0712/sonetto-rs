// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamchessEnemy {
    #[serde(rename = "behaviorId")]
    pub behavior_id: i32,
    #[serde(rename = "headImg")]
    pub head_img: String,
    #[serde(rename = "hp")]
    pub hp: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "passiveSkillIds")]
    pub passive_skill_ids: String,
    #[serde(rename = "skillDesc")]
    pub skill_desc: String,
    #[serde(rename = "skillIcon")]
    pub skill_icon: String,
    #[serde(rename = "specialAttr1")]
    pub special_attr1: String,
    #[serde(rename = "specialAttr2")]
    pub special_attr2: String,
    #[serde(rename = "specialAttr3")]
    pub special_attr3: String,
    #[serde(rename = "specialAttr4")]
    pub special_attr4: String,
    #[serde(rename = "specialAttr5")]
    pub special_attr5: String,
}

pub struct TeamchessEnemyTable {
    records: Vec<TeamchessEnemy>,
    by_id: HashMap<i32, usize>,
}

impl TeamchessEnemyTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<TeamchessEnemy> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&TeamchessEnemy> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[TeamchessEnemy] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TeamchessEnemy> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
