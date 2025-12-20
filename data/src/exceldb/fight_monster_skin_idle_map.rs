// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightMonsterSkinIdleMap {
    #[serde(rename = "freezeAnimName")]
    pub freeze_anim_name: String,
    #[serde(rename = "hitAnimName")]
    pub hit_anim_name: String,
    #[serde(rename = "idleAnimName")]
    pub idle_anim_name: String,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
    #[serde(rename = "sleepAnimName")]
    pub sleep_anim_name: String,
}

pub struct FightMonsterSkinIdleMapTable {
    records: Vec<FightMonsterSkinIdleMap>,
}

impl FightMonsterSkinIdleMapTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<FightMonsterSkinIdleMap> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[FightMonsterSkinIdleMap] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightMonsterSkinIdleMap> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
