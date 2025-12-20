// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalStorytask {
    #[serde(rename = "autoDrop")]
    pub auto_drop: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "desc2")]
    pub desc2: String,
    #[serde(rename = "desc3")]
    pub desc3: String,
    #[serde(rename = "desc4")]
    pub desc4: String,
    #[serde(rename = "dropShow")]
    pub drop_show: String,
    #[serde(rename = "eventID")]
    pub event_i_d: i32,
    #[serde(rename = "failCondition")]
    pub fail_condition: String,
    #[serde(rename = "group")]
    pub group: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "maxProgress")]
    pub max_progress: i32,
    #[serde(rename = "needAccept")]
    pub need_accept: i32,
    #[serde(rename = "prepose")]
    pub prepose: String,
    #[serde(rename = "progressCondition")]
    pub progress_condition: String,
    #[serde(rename = "step")]
    pub step: i32,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "track")]
    pub track: String,
    #[serde(rename = "uselessCondition")]
    pub useless_condition: String,
}

pub struct SurvivalStorytaskTable {
    records: Vec<SurvivalStorytask>,
    by_id: HashMap<i32, usize>,
}

impl SurvivalStorytaskTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SurvivalStorytask> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SurvivalStorytask> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SurvivalStorytask] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SurvivalStorytask> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
