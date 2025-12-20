// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalShop {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "pos1")]
    pub pos1: i32,
    #[serde(rename = "pos10")]
    pub pos10: i32,
    #[serde(rename = "pos11")]
    pub pos11: i32,
    #[serde(rename = "pos12")]
    pub pos12: i32,
    #[serde(rename = "pos13")]
    pub pos13: i32,
    #[serde(rename = "pos14")]
    pub pos14: i32,
    #[serde(rename = "pos15")]
    pub pos15: i32,
    #[serde(rename = "pos16")]
    pub pos16: i32,
    #[serde(rename = "pos17")]
    pub pos17: i32,
    #[serde(rename = "pos18")]
    pub pos18: i32,
    #[serde(rename = "pos19")]
    pub pos19: i32,
    #[serde(rename = "pos2")]
    pub pos2: i32,
    #[serde(rename = "pos20")]
    pub pos20: i32,
    #[serde(rename = "pos21")]
    pub pos21: i32,
    #[serde(rename = "pos22")]
    pub pos22: i32,
    #[serde(rename = "pos3")]
    pub pos3: i32,
    #[serde(rename = "pos4")]
    pub pos4: i32,
    #[serde(rename = "pos5")]
    pub pos5: i32,
    #[serde(rename = "pos6")]
    pub pos6: i32,
    #[serde(rename = "pos7")]
    pub pos7: i32,
    #[serde(rename = "pos8")]
    pub pos8: i32,
    #[serde(rename = "pos9")]
    pub pos9: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct SurvivalShopTable {
    records: Vec<SurvivalShop>,
    by_id: HashMap<i32, usize>,
}

impl SurvivalShopTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SurvivalShop> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SurvivalShop> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SurvivalShop] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SurvivalShop> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
