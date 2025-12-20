// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreRecommend {
    #[serde(rename = "adjustOrder")]
    pub adjust_order: String,
    #[serde(rename = "className")]
    pub class_name: String,
    #[serde(rename = "country")]
    pub country: Option<serde_json::Value>,
    #[serde(rename = "des")]
    pub des: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isCustomLoad")]
    pub is_custom_load: i32,
    #[serde(rename = "isOffline")]
    pub is_offline: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "offlineTime")]
    pub offline_time: String,
    #[serde(rename = "onlineTime")]
    pub online_time: String,
    #[serde(rename = "order")]
    pub order: i32,
    #[serde(rename = "prefab")]
    pub prefab: i32,
    #[serde(rename = "relations")]
    pub relations: String,
    #[serde(rename = "res")]
    pub res: String,
    #[serde(rename = "showOfflineTime")]
    pub show_offline_time: String,
    #[serde(rename = "showOnlineTime")]
    pub show_online_time: String,
    #[serde(rename = "systemJumpCode")]
    pub system_jump_code: String,
    #[serde(rename = "topDay")]
    pub top_day: i32,
    #[serde(rename = "topType")]
    pub top_type: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct StoreRecommendTable {
    records: Vec<StoreRecommend>,
    by_id: HashMap<i32, usize>,
}

impl StoreRecommendTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<StoreRecommend> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&StoreRecommend> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[StoreRecommend] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, StoreRecommend> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
