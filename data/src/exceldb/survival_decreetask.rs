// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalDecreetask {
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "desc2")]
    pub desc2: i32,
    #[serde(rename = "failCondition")]
    pub fail_condition: i32,
    #[serde(rename = "group")]
    pub group: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "maxProgress")]
    pub max_progress: String,
    #[serde(rename = "needAccept")]
    pub need_accept: String,
    #[serde(rename = "prepose")]
    pub prepose: String,
    #[serde(rename = "progressCondition")]
    pub progress_condition: i32,
    #[serde(rename = "seasons")]
    pub seasons: String,
    #[serde(rename = "step")]
    pub step: String,
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(rename = "versions")]
    pub versions: String,
}

pub struct SurvivalDecreetaskTable {
    records: Vec<SurvivalDecreetask>,
    by_id: HashMap<i32, usize>,
}

impl SurvivalDecreetaskTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SurvivalDecreetask> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SurvivalDecreetask> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SurvivalDecreetask] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SurvivalDecreetask> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
