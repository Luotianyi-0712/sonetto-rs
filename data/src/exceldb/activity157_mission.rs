// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity157Mission {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "area")]
    pub area: String,
    #[serde(rename = "elementId")]
    pub element_id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "linePrefab")]
    pub line_prefab: String,
    #[serde(rename = "missionId")]
    pub mission_id: i32,
    #[serde(rename = "order")]
    pub order: i32,
    #[serde(rename = "pos")]
    pub pos: String,
    #[serde(rename = "storyId")]
    pub story_id: i32,
}

pub struct Activity157MissionTable {
    records: Vec<Activity157Mission>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl Activity157MissionTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity157Mission> = if let Some(array) = value.as_array() {
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
                by_group.entry(record.group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_group,
        })
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &'_ Activity157Mission> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity157Mission] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity157Mission> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
