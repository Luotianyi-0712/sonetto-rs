// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalShelterMonsterAct {
    #[serde(rename = "fightId")]
    pub fight_id: i32,
    #[serde(rename = "monsterID")]
    pub monster_i_d: i32,
    #[serde(rename = "monsterSpeed")]
    pub monster_speed: i32,
    #[serde(rename = "round1")]
    pub round1: String,
    #[serde(rename = "round10")]
    pub round10: String,
    #[serde(rename = "round2")]
    pub round2: String,
    #[serde(rename = "round3")]
    pub round3: String,
    #[serde(rename = "round4")]
    pub round4: String,
    #[serde(rename = "round5")]
    pub round5: String,
    #[serde(rename = "round6")]
    pub round6: String,
    #[serde(rename = "round7")]
    pub round7: String,
    #[serde(rename = "round8")]
    pub round8: String,
    #[serde(rename = "round9")]
    pub round9: String,
}

pub struct SurvivalShelterMonsterActTable {
    records: Vec<SurvivalShelterMonsterAct>,
}

impl SurvivalShelterMonsterActTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SurvivalShelterMonsterAct> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[SurvivalShelterMonsterAct] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SurvivalShelterMonsterAct> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
