// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentCubeAttr {
    #[serde(rename = "absorb")]
    pub absorb: i32,
    #[serde(rename = "add_dmg")]
    pub add_dmg: i32,
    #[serde(rename = "atk")]
    pub atk: i32,
    #[serde(rename = "calculateType")]
    pub calculate_type: i32,
    #[serde(rename = "clutch")]
    pub clutch: i32,
    #[serde(rename = "cri")]
    pub cri: i32,
    #[serde(rename = "cri_def")]
    pub cri_def: i32,
    #[serde(rename = "cri_dmg")]
    pub cri_dmg: i32,
    #[serde(rename = "def")]
    pub def: i32,
    #[serde(rename = "defenseIgnore")]
    pub defense_ignore: i32,
    #[serde(rename = "drop_dmg")]
    pub drop_dmg: i32,
    #[serde(rename = "heal")]
    pub heal: i32,
    #[serde(rename = "hp")]
    pub hp: i32,
    #[serde(rename = "icon")]
    pub icon: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "mdef")]
    pub mdef: i32,
    #[serde(rename = "normalSkillRate")]
    pub normal_skill_rate: i32,
    #[serde(rename = "recri")]
    pub recri: i32,
    #[serde(rename = "revive")]
    pub revive: i32,
}

pub struct TalentCubeAttrTable {
    records: Vec<TalentCubeAttr>,
    by_id: HashMap<i32, usize>,
}

impl TalentCubeAttrTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<TalentCubeAttr> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&TalentCubeAttr> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[TalentCubeAttr] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TalentCubeAttr> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
