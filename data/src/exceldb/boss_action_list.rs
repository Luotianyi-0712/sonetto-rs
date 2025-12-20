// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossActionList {
    #[serde(rename = "actionId")]
    pub action_id: i32,
    #[serde(rename = "actionId1")]
    pub action_id1: String,
    #[serde(rename = "actionId10")]
    pub action_id10: String,
    #[serde(rename = "actionId2")]
    pub action_id2: String,
    #[serde(rename = "actionId3")]
    pub action_id3: String,
    #[serde(rename = "actionId4")]
    pub action_id4: String,
    #[serde(rename = "actionId5")]
    pub action_id5: String,
    #[serde(rename = "actionId6")]
    pub action_id6: String,
    #[serde(rename = "actionId7")]
    pub action_id7: String,
    #[serde(rename = "actionId8")]
    pub action_id8: String,
    #[serde(rename = "actionId9")]
    pub action_id9: String,
    #[serde(rename = "circle")]
    pub circle: i32,
}

pub struct BossActionListTable {
    records: Vec<BossActionList>,
}

impl BossActionListTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<BossActionList> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[BossActionList] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, BossActionList> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
