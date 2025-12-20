// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightSpEffectAlfTimeline {
    #[serde(rename = "skinId")]
    pub skin_id: i32,
    #[serde(rename = "timeline_2")]
    pub timeline_2: String,
    #[serde(rename = "timeline_3")]
    pub timeline_3: String,
    #[serde(rename = "timeline_4")]
    pub timeline_4: String,
}

pub struct FightSpEffectAlfTimelineTable {
    records: Vec<FightSpEffectAlfTimeline>,
}

impl FightSpEffectAlfTimelineTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<FightSpEffectAlfTimeline> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[FightSpEffectAlfTimeline] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightSpEffectAlfTimeline> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
