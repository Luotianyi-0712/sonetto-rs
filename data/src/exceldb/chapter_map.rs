// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterMap {
    #[serde(rename = "areaAudio")]
    pub area_audio: String,
    #[serde(rename = "chapterId")]
    pub chapter_id: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "effectAudio")]
    pub effect_audio: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "initPos")]
    pub init_pos: String,
    #[serde(rename = "mapIdGroup")]
    pub map_id_group: i32,
    #[serde(rename = "mapState")]
    pub map_state: i32,
    #[serde(rename = "playEffect")]
    pub play_effect: i32,
    #[serde(rename = "res")]
    pub res: String,
    #[serde(rename = "unlockCondition")]
    pub unlock_condition: String,
}

pub struct ChapterMapTable {
    records: Vec<ChapterMap>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl ChapterMapTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<ChapterMap> = if let Some(array) = value.as_array() {
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
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
                by_group.entry(record.map_id_group).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&ChapterMap> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &'_ ChapterMap> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[ChapterMap] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, ChapterMap> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
