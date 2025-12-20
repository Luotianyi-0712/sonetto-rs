// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightSpWuerlixiMonsterStarPositionOffset {
    #[serde(rename = "monsterId")]
    pub monster_id: i32,
    #[serde(rename = "offsetX")]
    pub offset_x: f32,
    #[serde(rename = "offsetY")]
    pub offset_y: f32,
}

pub struct FightSpWuerlixiMonsterStarPositionOffsetTable {
    records: Vec<FightSpWuerlixiMonsterStarPositionOffset>,
}

impl FightSpWuerlixiMonsterStarPositionOffsetTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<FightSpWuerlixiMonsterStarPositionOffset> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[FightSpWuerlixiMonsterStarPositionOffset] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightSpWuerlixiMonsterStarPositionOffset> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
