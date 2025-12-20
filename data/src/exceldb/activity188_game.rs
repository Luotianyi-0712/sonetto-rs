// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity188Game {
    #[serde(rename = "abilityIds")]
    pub ability_ids: String,
    #[serde(rename = "abilityPool")]
    pub ability_pool: String,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "bossAbilityIds")]
    pub boss_ability_ids: String,
    #[serde(rename = "bossAbilityPool")]
    pub boss_ability_pool: String,
    #[serde(rename = "bossCount")]
    pub boss_count: i32,
    #[serde(rename = "bossHp")]
    pub boss_hp: i32,
    #[serde(rename = "bossHurt")]
    pub boss_hurt: i32,
    #[serde(rename = "cardBuild")]
    pub card_build: String,
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "difficult")]
    pub difficult: i32,
    #[serde(rename = "hp")]
    pub hp: i32,
    #[serde(rename = "hurt")]
    pub hurt: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "passRound")]
    pub pass_round: String,
    #[serde(rename = "portrait")]
    pub portrait: String,
    #[serde(rename = "readNum")]
    pub read_num: i32,
    #[serde(rename = "round")]
    pub round: i32,
    #[serde(rename = "rowColumn")]
    pub row_column: String,
}

pub struct Activity188GameTable {
    records: Vec<Activity188Game>,
    by_id: HashMap<i32, usize>,
}

impl Activity188GameTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity188Game> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity188Game> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity188Game] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity188Game> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
