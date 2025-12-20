// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity130Decrypt {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "answer")]
    pub answer: String,
    #[serde(rename = "errorTip")]
    pub error_tip: String,
    #[serde(rename = "extStarCondition")]
    pub ext_star_condition: String,
    #[serde(rename = "extStarDesc")]
    pub ext_star_desc: String,
    #[serde(rename = "maxOper")]
    pub max_oper: i32,
    #[serde(rename = "maxStep")]
    pub max_step: i32,
    #[serde(rename = "operGroupId")]
    pub oper_group_id: i32,
    #[serde(rename = "puzzleId")]
    pub puzzle_id: i32,
    #[serde(rename = "puzzleMapId")]
    pub puzzle_map_id: i32,
    #[serde(rename = "puzzleTip")]
    pub puzzle_tip: String,
    #[serde(rename = "puzzleTxt")]
    pub puzzle_txt: String,
    #[serde(rename = "puzzleType")]
    pub puzzle_type: i32,
}

pub struct Activity130DecryptTable {
    records: Vec<Activity130Decrypt>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl Activity130DecryptTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity130Decrypt> = if let Some(array) = value.as_array() {
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
        
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
                by_group.entry(record.oper_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_group,
        })
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &'_ Activity130Decrypt> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity130Decrypt] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity130Decrypt> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
