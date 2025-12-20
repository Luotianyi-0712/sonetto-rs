// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity174Bag {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "bagDesc")]
    pub bag_desc: String,
    #[serde(rename = "bagTitle")]
    pub bag_title: String,
    #[serde(rename = "collectionNum")]
    pub collection_num: i32,
    #[serde(rename = "collectionParam")]
    pub collection_param: String,
    #[serde(rename = "collectionType")]
    pub collection_type: String,
    #[serde(rename = "costCoin")]
    pub cost_coin: i32,
    #[serde(rename = "enhanceNum")]
    pub enhance_num: i32,
    #[serde(rename = "enhanceParam")]
    pub enhance_param: String,
    #[serde(rename = "enhanceType")]
    pub enhance_type: String,
    #[serde(rename = "heroNum")]
    pub hero_num: i32,
    #[serde(rename = "heroParam")]
    pub hero_param: String,
    #[serde(rename = "heroType")]
    pub hero_type: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "quality")]
    pub quality: String,
    #[serde(rename = "season")]
    pub season: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

pub struct Activity174BagTable {
    records: Vec<Activity174Bag>,
    by_id: HashMap<i32, usize>,
}

impl Activity174BagTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity174Bag> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity174Bag> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity174Bag] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity174Bag> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
