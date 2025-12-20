// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroTrialAttr {
    #[serde(rename = "activeSkill")]
    pub active_skill: String,
    #[serde(rename = "addDmg")]
    pub add_dmg: i32,
    #[serde(rename = "attack")]
    pub attack: i32,
    #[serde(rename = "cri")]
    pub cri: i32,
    #[serde(rename = "criDef")]
    pub cri_def: i32,
    #[serde(rename = "criDmg")]
    pub cri_dmg: i32,
    #[serde(rename = "defense")]
    pub defense: i32,
    #[serde(rename = "dropDmg")]
    pub drop_dmg: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "life")]
    pub life: i32,
    #[serde(rename = "mdefense")]
    pub mdefense: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "noShowExSkill")]
    pub no_show_ex_skill: i32,
    #[serde(rename = "passiveSkill")]
    pub passive_skill: String,
    #[serde(rename = "recri")]
    pub recri: i32,
    #[serde(rename = "technic")]
    pub technic: i32,
    #[serde(rename = "uniqueSkill")]
    pub unique_skill: i32,
}

pub struct HeroTrialAttrTable {
    records: Vec<HeroTrialAttr>,
    by_id: HashMap<i32, usize>,
}

impl HeroTrialAttrTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<HeroTrialAttr> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&HeroTrialAttr> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[HeroTrialAttr] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, HeroTrialAttr> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
