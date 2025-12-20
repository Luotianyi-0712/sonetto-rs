// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinWeatherParam {
    #[serde(rename = "bloomColor1")]
    pub bloom_color1: String,
    #[serde(rename = "bloomColor2")]
    pub bloom_color2: String,
    #[serde(rename = "bloomColor3")]
    pub bloom_color3: String,
    #[serde(rename = "bloomColor4")]
    pub bloom_color4: String,
    #[serde(rename = "emissionColor1")]
    pub emission_color1: String,
    #[serde(rename = "emissionColor2")]
    pub emission_color2: String,
    #[serde(rename = "emissionColor3")]
    pub emission_color3: String,
    #[serde(rename = "emissionColor4")]
    pub emission_color4: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mainColor1")]
    pub main_color1: String,
    #[serde(rename = "mainColor2")]
    pub main_color2: String,
    #[serde(rename = "mainColor3")]
    pub main_color3: String,
    #[serde(rename = "mainColor4")]
    pub main_color4: String,
}

pub struct SkinWeatherParamTable {
    records: Vec<SkinWeatherParam>,
    by_id: HashMap<i32, usize>,
}

impl SkinWeatherParamTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SkinWeatherParam> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SkinWeatherParam> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SkinWeatherParam] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SkinWeatherParam> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
