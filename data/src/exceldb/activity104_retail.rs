// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity104Retail {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "enemyTag")]
    pub enemy_tag: String,
    #[serde(rename = "equipRareWeight")]
    pub equip_rare_weight: String,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "retailEpisodeIdPool")]
    pub retail_episode_id_pool: String,
    #[serde(rename = "stage")]
    pub stage: i32,
}

pub struct Activity104RetailTable {
    records: Vec<Activity104Retail>,
}

impl Activity104RetailTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity104Retail> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity104Retail] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity104Retail> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
