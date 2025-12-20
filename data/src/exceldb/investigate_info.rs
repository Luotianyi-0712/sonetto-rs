// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestigateInfo {
    #[serde(rename = "clueNumber")]
    pub clue_number: i32,
    #[serde(rename = "conclusionBg")]
    pub conclusion_bg: String,
    #[serde(rename = "conclusionDesc")]
    pub conclusion_desc: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "entrance")]
    pub entrance: i32,
    #[serde(rename = "episode")]
    pub episode: i32,
    #[serde(rename = "group")]
    pub group: i32,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "unlockDesc")]
    pub unlock_desc: String,
}

pub struct InvestigateInfoTable {
    records: Vec<InvestigateInfo>,
    by_id: HashMap<i32, usize>,
}

impl InvestigateInfoTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<InvestigateInfo> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&InvestigateInfo> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[InvestigateInfo] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, InvestigateInfo> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
