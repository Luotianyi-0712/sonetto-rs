// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RougeTalentBranch {
    #[serde(rename = "attribute")]
    pub attribute: String,
    #[serde(rename = "before")]
    pub before: String,
    #[serde(rename = "cost")]
    pub cost: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isOrigin")]
    pub is_origin: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "pos")]
    pub pos: String,
    #[serde(rename = "special")]
    pub special: String,
    #[serde(rename = "talent")]
    pub talent: i32,
}

pub struct RougeTalentBranchTable {
    records: Vec<RougeTalentBranch>,
    by_id: HashMap<i32, usize>,
}

impl RougeTalentBranchTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RougeTalentBranch> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&RougeTalentBranch> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[RougeTalentBranch] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RougeTalentBranch> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
