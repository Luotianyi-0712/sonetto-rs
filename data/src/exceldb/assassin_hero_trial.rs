// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssassinHeroTrial {
    #[serde(rename = "assassinHeroId")]
    pub assassin_hero_id: i32,
    #[serde(rename = "career")]
    pub career: i32,
    #[serde(rename = "entityIcon")]
    pub entity_icon: String,
    #[serde(rename = "heroIcon")]
    pub hero_icon: String,
    #[serde(rename = "heroImg")]
    pub hero_img: String,
    #[serde(rename = "model")]
    pub model: i32,
    #[serde(rename = "secondCareer")]
    pub second_career: i32,
    #[serde(rename = "unlock")]
    pub unlock: String,
}

pub struct AssassinHeroTrialTable {
    records: Vec<AssassinHeroTrial>,
}

impl AssassinHeroTrialTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<AssassinHeroTrial> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[AssassinHeroTrial] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, AssassinHeroTrial> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
