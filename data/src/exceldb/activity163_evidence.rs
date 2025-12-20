// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity163Evidence {
    #[serde(rename = "conditionStr")]
    pub condition_str: String,
    #[serde(rename = "dialogGroupType")]
    pub dialog_group_type: i32,
    #[serde(rename = "errorTip")]
    pub error_tip: i32,
    #[serde(rename = "evidenceId")]
    pub evidence_id: i32,
    #[serde(rename = "puzzleTxt")]
    pub puzzle_txt: String,
    #[serde(rename = "showFusion")]
    pub show_fusion: i32,
    #[serde(rename = "tips")]
    pub tips: i32,
}

pub struct Activity163EvidenceTable {
    records: Vec<Activity163Evidence>,
}

impl Activity163EvidenceTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity163Evidence> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity163Evidence] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity163Evidence> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
