// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroUpgradeOptions {
    #[serde(rename = "addBuff")]
    pub add_buff: String,
    #[serde(rename = "addPassiveSkill")]
    pub add_passive_skill: String,
    #[serde(rename = "delBuff")]
    pub del_buff: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "replaceBigSkill")]
    pub replace_big_skill: i32,
    #[serde(rename = "replacePassiveSkill")]
    pub replace_passive_skill: String,
    #[serde(rename = "replaceSkillGroup1")]
    pub replace_skill_group1: String,
    #[serde(rename = "replaceSkillGroup2")]
    pub replace_skill_group2: String,
    #[serde(rename = "showSkillId")]
    pub show_skill_id: i32,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "unlockCondition")]
    pub unlock_condition: String,
}

pub struct HeroUpgradeOptionsTable {
    records: Vec<HeroUpgradeOptions>,
    by_id: HashMap<i32, usize>,
}

impl HeroUpgradeOptionsTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<HeroUpgradeOptions> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&HeroUpgradeOptions> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[HeroUpgradeOptions] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, HeroUpgradeOptions> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
