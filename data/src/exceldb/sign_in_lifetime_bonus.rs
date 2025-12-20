// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignInLifetimeBonus {
    #[serde(rename = "bonus")]
    pub bonus: String,
    #[serde(rename = "logindaysid")]
    pub logindaysid: i32,
    #[serde(rename = "stageid")]
    pub stageid: i32,
    #[serde(rename = "stagetitle")]
    pub stagetitle: String,
}

pub struct SignInLifetimeBonusTable {
    records: Vec<SignInLifetimeBonus>,
}

impl SignInLifetimeBonusTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SignInLifetimeBonus> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[SignInLifetimeBonus] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SignInLifetimeBonus> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
