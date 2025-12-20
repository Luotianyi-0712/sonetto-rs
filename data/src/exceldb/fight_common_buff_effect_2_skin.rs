// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightCommonBuffEffect2Skin {
    #[serde(rename = "audio")]
    pub audio: i32,
    #[serde(rename = "buffId")]
    pub buff_id: i32,
    #[serde(rename = "duration")]
    pub duration: f32,
    #[serde(rename = "effectHang")]
    pub effect_hang: String,
    #[serde(rename = "effectPath")]
    pub effect_path: String,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
}

pub struct FightCommonBuffEffect2SkinTable {
    records: Vec<FightCommonBuffEffect2Skin>,
}

impl FightCommonBuffEffect2SkinTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<FightCommonBuffEffect2Skin> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[FightCommonBuffEffect2Skin] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightCommonBuffEffect2Skin> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
