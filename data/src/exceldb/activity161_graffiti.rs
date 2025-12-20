// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity161Graffiti {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "brushSize")]
    pub brush_size: i32,
    #[serde(rename = "dialogGroupId")]
    pub dialog_group_id: i32,
    #[serde(rename = "elementId")]
    pub element_id: i32,
    #[serde(rename = "finishDesc")]
    pub finish_desc: String,
    #[serde(rename = "finishRate")]
    pub finish_rate: i32,
    #[serde(rename = "finishTitle")]
    pub finish_title: String,
    #[serde(rename = "finishTitleEn")]
    pub finish_title_en: String,
    #[serde(rename = "mainElementCd")]
    pub main_element_cd: i32,
    #[serde(rename = "mainElementId")]
    pub main_element_id: i32,
    #[serde(rename = "picture")]
    pub picture: String,
    #[serde(rename = "preMainElementIds")]
    pub pre_main_element_ids: String,
    #[serde(rename = "sort")]
    pub sort: i32,
    #[serde(rename = "subElementIds")]
    pub sub_element_ids: String,
}

pub struct Activity161GraffitiTable {
    records: Vec<Activity161Graffiti>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl Activity161GraffitiTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity161Graffiti> = if let Some(array) = value.as_array() {
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
                by_group.entry(record.dialog_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_group,
        })
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &'_ Activity161Graffiti> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity161Graffiti] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity161Graffiti> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
