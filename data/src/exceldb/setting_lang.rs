// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingLang {
    #[serde(rename = "fontasset1")]
    pub fontasset1: String,
    #[serde(rename = "fontasset2")]
    pub fontasset2: String,
    #[serde(rename = "lang")]
    pub lang: String,
    #[serde(rename = "shortcuts")]
    pub shortcuts: String,
    #[serde(rename = "textfontasset1")]
    pub textfontasset1: String,
    #[serde(rename = "textfontasset2")]
    pub textfontasset2: String,
}

pub struct SettingLangTable {
    records: Vec<SettingLang>,
}

impl SettingLangTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SettingLang> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[SettingLang] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SettingLang> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
