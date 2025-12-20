// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamchessCharacter {
    #[serde(rename = "activeSkillIds")]
    pub active_skill_ids: String,
    #[serde(rename = "hp")]
    pub hp: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "initDiamonds")]
    pub init_diamonds: String,
    #[serde(rename = "initPower")]
    pub init_power: i32,
    #[serde(rename = "maxPowerLimit")]
    pub max_power_limit: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "passiveSkillIds")]
    pub passive_skill_ids: String,
    #[serde(rename = "resPic")]
    pub res_pic: String,
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

pub struct TeamchessCharacterTable {
    records: Vec<TeamchessCharacter>,
    by_id: HashMap<i32, usize>,
}

impl TeamchessCharacterTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<TeamchessCharacter> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&TeamchessCharacter> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[TeamchessCharacter] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TeamchessCharacter> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
