// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoChessMaster {
    #[serde(rename = "hp")]
    pub hp: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "illustrationShow")]
    pub illustration_show: i32,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "isSelf")]
    pub is_self: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "roundTriggerCountLimit")]
    pub round_trigger_count_limit: i32,
    #[serde(rename = "skillDesc")]
    pub skill_desc: String,
    #[serde(rename = "skillIcon")]
    pub skill_icon: String,
    #[serde(rename = "skillId")]
    pub skill_id: i32,
    #[serde(rename = "skillLockDesc")]
    pub skill_lock_desc: String,
    #[serde(rename = "skillName")]
    pub skill_name: String,
    #[serde(rename = "skillProgressDesc")]
    pub skill_progress_desc: String,
    #[serde(rename = "spUdimo")]
    pub sp_udimo: i32,
    #[serde(rename = "spUdimoGoodsId")]
    pub sp_udimo_goods_id: i32,
    #[serde(rename = "totalTriggerCountLimit")]
    pub total_trigger_count_limit: i32,
    #[serde(rename = "udimoCase")]
    pub udimo_case: String,
    #[serde(rename = "unlockSkill")]
    pub unlock_skill: i32,
}

pub struct AutoChessMasterTable {
    records: Vec<AutoChessMaster>,
    by_id: HashMap<i32, usize>,
}

impl AutoChessMasterTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<AutoChessMaster> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&AutoChessMaster> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[AutoChessMaster] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, AutoChessMaster> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
