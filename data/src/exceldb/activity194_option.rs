// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity194Option {
    #[serde(rename = "conditionDesc")]
    pub condition_desc: String,
    #[serde(rename = "effect")]
    pub effect: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "optionDesc")]
    pub option_desc: String,
    #[serde(rename = "optionId")]
    pub option_id: i32,
    #[serde(rename = "optionRestriction")]
    pub option_restriction: String,
    #[serde(rename = "optionResultId")]
    pub option_result_id: i32,
}

pub struct Activity194OptionTable {
    records: Vec<Activity194Option>,
}

impl Activity194OptionTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity194Option> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity194Option] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity194Option> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
