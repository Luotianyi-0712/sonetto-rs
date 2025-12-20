// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalTalentGroup {
    #[serde(rename = "activeIcon")]
    pub active_icon: String,
    #[serde(rename = "align")]
    pub align: String,
    #[serde(rename = "border")]
    pub border: String,
    #[serde(rename = "choose")]
    pub choose: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "folder")]
    pub folder: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "initTalents")]
    pub init_talents: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "seasons")]
    pub seasons: String,
    #[serde(rename = "slot1")]
    pub slot1: i32,
    #[serde(rename = "slot2")]
    pub slot2: i32,
    #[serde(rename = "slot3")]
    pub slot3: i32,
    #[serde(rename = "slot4")]
    pub slot4: i32,
    #[serde(rename = "slot5")]
    pub slot5: i32,
    #[serde(rename = "slot6")]
    pub slot6: i32,
    #[serde(rename = "slot7")]
    pub slot7: i32,
    #[serde(rename = "slot8")]
    pub slot8: i32,
    #[serde(rename = "versions")]
    pub versions: String,
}

pub struct SurvivalTalentGroupTable {
    records: Vec<SurvivalTalentGroup>,
    by_id: HashMap<i32, usize>,
}

impl SurvivalTalentGroupTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SurvivalTalentGroup> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SurvivalTalentGroup> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SurvivalTalentGroup] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SurvivalTalentGroup> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
