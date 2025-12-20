// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryModeBattleField {
    #[serde(rename = "chapterId")]
    pub chapter_id: i32,
    #[serde(rename = "chapterMapIds")]
    pub chapter_map_ids: String,
    #[serde(rename = "episodeIds")]
    pub episode_ids: String,
    #[serde(rename = "fieldId")]
    pub field_id: i32,
    #[serde(rename = "heroGroupTypeId")]
    pub hero_group_type_id: i32,
}

pub struct StoryModeBattleFieldTable {
    records: Vec<StoryModeBattleField>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl StoryModeBattleFieldTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<StoryModeBattleField> = if let Some(array) = value.as_array() {
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
                by_group.entry(record.hero_group_type_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_group,
        })
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &'_ StoryModeBattleField> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[StoryModeBattleField] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, StoryModeBattleField> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
