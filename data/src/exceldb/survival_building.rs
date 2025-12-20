// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalBuilding {
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "destruction")]
    pub destruction: i32,
    #[serde(rename = "effect")]
    pub effect: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "lvUpCost")]
    pub lv_up_cost: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "repairCost")]
    pub repair_cost: String,
    #[serde(rename = "ruins")]
    pub ruins: String,
    #[serde(rename = "sort")]
    pub sort: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "unName")]
    pub un_name: i32,
    #[serde(rename = "unlockCondition")]
    pub unlock_condition: String,
}

pub struct SurvivalBuildingTable {
    records: Vec<SurvivalBuilding>,
    by_id: HashMap<i32, usize>,
}

impl SurvivalBuildingTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SurvivalBuilding> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SurvivalBuilding> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SurvivalBuilding] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SurvivalBuilding> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
