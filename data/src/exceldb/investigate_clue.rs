// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestigateClue {
    #[serde(rename = "defaultUnlock")]
    pub default_unlock: i32,
    #[serde(rename = "detailedDesc")]
    pub detailed_desc: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "infoID")]
    pub info_i_d: i32,
    #[serde(rename = "mapElement")]
    pub map_element: i32,
    #[serde(rename = "mapRes")]
    pub map_res: String,
    #[serde(rename = "mapResLocked")]
    pub map_res_locked: String,
    #[serde(rename = "relatedDesc")]
    pub related_desc: String,
    #[serde(rename = "res")]
    pub res: String,
}

pub struct InvestigateClueTable {
    records: Vec<InvestigateClue>,
    by_id: HashMap<i32, usize>,
}

impl InvestigateClueTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<InvestigateClue> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&InvestigateClue> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[InvestigateClue] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, InvestigateClue> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
