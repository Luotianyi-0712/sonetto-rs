// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillBufftype {
    #[serde(rename = "aniSort")]
    pub ani_sort: i32,
    #[serde(rename = "cannotRemove")]
    pub cannot_remove: bool,
    #[serde(rename = "dontShowFloat")]
    pub dont_show_float: i32,
    #[serde(rename = "excludeTypes")]
    pub exclude_types: String,
    #[serde(rename = "group")]
    pub group: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "includeTypes")]
    pub include_types: String,
    #[serde(rename = "matSort")]
    pub mat_sort: i32,
    #[serde(rename = "playEffect")]
    pub play_effect: i32,
    #[serde(rename = "removeNum")]
    pub remove_num: i32,
    #[serde(rename = "skipDelay")]
    pub skip_delay: i32,
    #[serde(rename = "takeAct")]
    pub take_act: String,
    #[serde(rename = "takeStage")]
    pub take_stage: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct SkillBufftypeTable {
    records: Vec<SkillBufftype>,
    by_id: HashMap<i32, usize>,
}

impl SkillBufftypeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SkillBufftype> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SkillBufftype> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SkillBufftype] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SkillBufftype> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
