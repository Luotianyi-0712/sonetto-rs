// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalHardness {
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "extendScoreFix")]
    pub extend_score_fix: i32,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isShow")]
    pub is_show: i32,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "lockDesc")]
    pub lock_desc: String,
    #[serde(rename = "optional")]
    pub optional: i32,
    #[serde(rename = "scoreRate")]
    pub score_rate: i32,
    #[serde(rename = "seasons")]
    pub seasons: String,
    #[serde(rename = "subtype")]
    pub subtype: i32,
    #[serde(rename = "titile")]
    pub titile: String,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "versions")]
    pub versions: String,
}

pub struct SurvivalHardnessTable {
    records: Vec<SurvivalHardness>,
    by_id: HashMap<i32, usize>,
}

impl SurvivalHardnessTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SurvivalHardness> = if let Some(array) = value.as_array() {
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
        
        let mut by_id = HashMap::with_capacity(records.len());
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
        }
        
        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&SurvivalHardness> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SurvivalHardness] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SurvivalHardness> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
