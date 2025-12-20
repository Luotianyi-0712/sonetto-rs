// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuckyBagHeroes {
    #[serde(rename = "bagId")]
    pub bag_id: i32,
    #[serde(rename = "heroChoices")]
    pub hero_choices: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "poolId")]
    pub pool_id: i32,
    #[serde(rename = "sceneIcon")]
    pub scene_icon: String,
}

pub struct LuckyBagHeroesTable {
    records: Vec<LuckyBagHeroes>,
}

impl LuckyBagHeroesTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<LuckyBagHeroes> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[LuckyBagHeroes] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, LuckyBagHeroes> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
