// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RougeStyle {
    #[serde(rename = "activeSkills")]
    pub active_skills: String,
    #[serde(rename = "capacity")]
    pub capacity: i32,
    #[serde(rename = "coin")]
    pub coin: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "halfCost")]
    pub half_cost: i32,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "layoutId")]
    pub layout_id: i32,
    #[serde(rename = "mapSkills")]
    pub map_skills: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "passiveSkillDescs")]
    pub passive_skill_descs: String,
    #[serde(rename = "passiveSkillDescs2")]
    pub passive_skill_descs2: String,
    #[serde(rename = "power")]
    pub power: i32,
    #[serde(rename = "powerLimit")]
    pub power_limit: i32,
    #[serde(rename = "season")]
    pub season: i32,
    #[serde(rename = "talentPointGroup")]
    pub talent_point_group: String,
    #[serde(rename = "talentSkill")]
    pub talent_skill: String,
    #[serde(rename = "unlockParam")]
    pub unlock_param: String,
    #[serde(rename = "unlockType")]
    pub unlock_type: i32,
    #[serde(rename = "version")]
    pub version: String,
}

pub struct RougeStyleTable {
    records: Vec<RougeStyle>,
    by_id: HashMap<i32, usize>,
}

impl RougeStyleTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RougeStyle> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&RougeStyle> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[RougeStyle] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RougeStyle> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
