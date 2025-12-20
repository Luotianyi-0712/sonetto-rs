// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CritterHeroPreference {
    #[serde(rename = "addIncrRate")]
    pub add_incr_rate: i32,
    #[serde(rename = "attachAttribute")]
    pub attach_attribute: String,
    #[serde(rename = "attachEventId")]
    pub attach_event_id: i32,
    #[serde(rename = "attachOption")]
    pub attach_option: String,
    #[serde(rename = "attachStoryId")]
    pub attach_story_id: i32,
    #[serde(rename = "critterIcon")]
    pub critter_icon: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "effectAttribute")]
    pub effect_attribute: i32,
    #[serde(rename = "heroId")]
    pub hero_id: i32,
    #[serde(rename = "preferenceType")]
    pub preference_type: i32,
    #[serde(rename = "preferenceValue")]
    pub preference_value: String,
    #[serde(rename = "spEventId")]
    pub sp_event_id: i32,
}

pub struct CritterHeroPreferenceTable {
    records: Vec<CritterHeroPreference>,
}

impl CritterHeroPreferenceTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<CritterHeroPreference> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[CritterHeroPreference] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CritterHeroPreference> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
