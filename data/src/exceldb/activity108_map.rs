// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity108Map {
    #[serde(rename = "actContent")]
    pub act_content: String,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "consignor")]
    pub consignor: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "cookie")]
    pub cookie: i32,
    #[serde(rename = "cookieContent")]
    pub cookie_content: String,
    #[serde(rename = "endContent")]
    pub end_content: String,
    #[serde(rename = "enemyInfo")]
    pub enemy_info: String,
    #[serde(rename = "enemyTitle")]
    pub enemy_title: String,
    #[serde(rename = "exhibits")]
    pub exhibits: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "initScore")]
    pub init_score: i32,
    #[serde(rename = "monsterIcon")]
    pub monster_icon: String,
    #[serde(rename = "offlineDay")]
    pub offline_day: i32,
    #[serde(rename = "onlineDay")]
    pub online_day: i32,
    #[serde(rename = "preId")]
    pub pre_id: i32,
    #[serde(rename = "threat")]
    pub threat: i32,
    #[serde(rename = "title")]
    pub title: String,
}

pub struct Activity108MapTable {
    records: Vec<Activity108Map>,
    by_id: HashMap<i32, usize>,
}

impl Activity108MapTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity108Map> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity108Map> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity108Map] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity108Map> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
