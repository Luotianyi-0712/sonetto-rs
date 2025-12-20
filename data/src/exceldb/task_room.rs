// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRoom {
    #[serde(rename = "bonus")]
    pub bonus: String,
    #[serde(rename = "bonusIcon")]
    pub bonus_icon: String,
    #[serde(rename = "bonusMail")]
    pub bonus_mail: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isOnline")]
    pub is_online: i32,
    #[serde(rename = "listenerParam")]
    pub listener_param: String,
    #[serde(rename = "listenerType")]
    pub listener_type: String,
    #[serde(rename = "maxFinishCount")]
    pub max_finish_count: i32,
    #[serde(rename = "maxProgress")]
    pub max_progress: i32,
    #[serde(rename = "minType")]
    pub min_type: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "needAccept")]
    pub need_accept: i32,
    #[serde(rename = "needReset")]
    pub need_reset: bool,
    #[serde(rename = "onceBonus")]
    pub once_bonus: String,
    #[serde(rename = "openLimit")]
    pub open_limit: String,
    #[serde(rename = "order")]
    pub order: String,
    #[serde(rename = "params")]
    pub params: String,
    #[serde(rename = "prepose")]
    pub prepose: String,
    #[serde(rename = "tips")]
    pub tips: String,
}

pub struct TaskRoomTable {
    records: Vec<TaskRoom>,
    by_id: HashMap<i32, usize>,
}

impl TaskRoomTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<TaskRoom> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&TaskRoom> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[TaskRoom] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TaskRoom> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
