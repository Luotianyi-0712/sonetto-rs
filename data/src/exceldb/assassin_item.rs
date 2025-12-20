// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssassinItem {
    #[serde(rename = "costPoint")]
    pub cost_point: i32,
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "effect")]
    pub effect: String,
    #[serde(rename = "fightEffDesc")]
    pub fight_eff_desc: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "itemType")]
    pub item_type: i32,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "param")]
    pub param: String,
    #[serde(rename = "range")]
    pub range: String,
    #[serde(rename = "roundLimit")]
    pub round_limit: i32,
    #[serde(rename = "skillId")]
    pub skill_id: i32,
    #[serde(rename = "stealthEffDesc")]
    pub stealth_eff_desc: String,
    #[serde(rename = "target")]
    pub target: String,
    #[serde(rename = "targetCheck")]
    pub target_check: String,
    #[serde(rename = "targetEff")]
    pub target_eff: i32,
    #[serde(rename = "unlock")]
    pub unlock: String,
}

pub struct AssassinItemTable {
    records: Vec<AssassinItem>,
}

impl AssassinItemTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<AssassinItem> = if let Some(array) = value.as_array() {
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
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[AssassinItem] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, AssassinItem> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
