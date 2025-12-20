// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightSpEffectWuerlixiTimeline {
    #[serde(rename = "enemySideTimeline")]
    pub enemy_side_timeline: String,
    #[serde(rename = "mySideTimeline")]
    pub my_side_timeline: String,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
}

pub struct FightSpEffectWuerlixiTimelineTable {
    records: Vec<FightSpEffectWuerlixiTimeline>,
}

impl FightSpEffectWuerlixiTimelineTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<FightSpEffectWuerlixiTimeline> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[FightSpEffectWuerlixiTimeline] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightSpEffectWuerlixiTimeline> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
