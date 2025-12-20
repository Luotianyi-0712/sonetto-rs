// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightSkinDeadPerformance {
    #[serde(rename = "actParam1")]
    pub act_param1: String,
    #[serde(rename = "actParam2")]
    pub act_param2: String,
    #[serde(rename = "actParam3")]
    pub act_param3: String,
    #[serde(rename = "actParam4")]
    pub act_param4: String,
    #[serde(rename = "actParam5")]
    pub act_param5: String,
    #[serde(rename = "actParam6")]
    pub act_param6: String,
    #[serde(rename = "actParam7")]
    pub act_param7: String,
    #[serde(rename = "actParam8")]
    pub act_param8: String,
    #[serde(rename = "actType1")]
    pub act_type1: i32,
    #[serde(rename = "actType2")]
    pub act_type2: i32,
    #[serde(rename = "actType3")]
    pub act_type3: i32,
    #[serde(rename = "actType4")]
    pub act_type4: i32,
    #[serde(rename = "actType5")]
    pub act_type5: i32,
    #[serde(rename = "actType6")]
    pub act_type6: i32,
    #[serde(rename = "actType7")]
    pub act_type7: i32,
    #[serde(rename = "actType8")]
    pub act_type8: i32,
    #[serde(rename = "id")]
    pub id: i32,
}

pub struct FightSkinDeadPerformanceTable {
    records: Vec<FightSkinDeadPerformance>,
    by_id: HashMap<i32, usize>,
}

impl FightSkinDeadPerformanceTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<FightSkinDeadPerformance> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&FightSkinDeadPerformance> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[FightSkinDeadPerformance] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightSkinDeadPerformance> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
