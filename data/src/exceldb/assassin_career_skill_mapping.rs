// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssassinCareerSkillMapping {
    #[serde(rename = "addSkill")]
    pub add_skill: i32,
    #[serde(rename = "assassinHeroId")]
    pub assassin_hero_id: i32,
    #[serde(rename = "careerId")]
    pub career_id: i32,
    #[serde(rename = "cost")]
    pub cost: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "effect")]
    pub effect: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "prerequisite")]
    pub prerequisite: String,
    #[serde(rename = "range")]
    pub range: String,
    #[serde(rename = "roundLimit")]
    pub round_limit: i32,
    #[serde(rename = "target")]
    pub target: String,
    #[serde(rename = "targetCheck")]
    pub target_check: String,
    #[serde(rename = "targetEff")]
    pub target_eff: i32,
    #[serde(rename = "timesLimit")]
    pub times_limit: i32,
    #[serde(rename = "triggerNode")]
    pub trigger_node: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

pub struct AssassinCareerSkillMappingTable {
    records: Vec<AssassinCareerSkillMapping>,
    by_id: HashMap<i32, usize>,
}

impl AssassinCareerSkillMappingTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<AssassinCareerSkillMapping> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&AssassinCareerSkillMapping> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[AssassinCareerSkillMapping] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, AssassinCareerSkillMapping> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
