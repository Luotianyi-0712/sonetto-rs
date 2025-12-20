// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actvity186MilestoneBonus {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "bonus")]
    pub bonus: String,
    #[serde(rename = "coinNum")]
    pub coin_num: i32,
    #[serde(rename = "isLoopBonus")]
    pub is_loop_bonus: bool,
    #[serde(rename = "isSpBonus")]
    pub is_sp_bonus: bool,
    #[serde(rename = "loopBonusIntervalNum")]
    pub loop_bonus_interval_num: i32,
    #[serde(rename = "rewardId")]
    pub reward_id: i32,
}

pub struct Actvity186MilestoneBonusTable {
    records: Vec<Actvity186MilestoneBonus>,
}

impl Actvity186MilestoneBonusTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Actvity186MilestoneBonus> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Actvity186MilestoneBonus] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Actvity186MilestoneBonus> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
