// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actvity186MiniGameQuestion {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "answer1")]
    pub answer1: String,
    #[serde(rename = "answer2")]
    pub answer2: String,
    #[serde(rename = "answer3")]
    pub answer3: String,
    #[serde(rename = "feedback1")]
    pub feedback1: String,
    #[serde(rename = "feedback2")]
    pub feedback2: String,
    #[serde(rename = "feedback3")]
    pub feedback3: String,
    #[serde(rename = "hanzhangline4")]
    pub hanzhangline4: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "question")]
    pub question: String,
    #[serde(rename = "rewardId1")]
    pub reward_id1: i32,
    #[serde(rename = "rewardId2")]
    pub reward_id2: i32,
    #[serde(rename = "rewardId3")]
    pub reward_id3: i32,
    #[serde(rename = "sort")]
    pub sort: i32,
}

pub struct Actvity186MiniGameQuestionTable {
    records: Vec<Actvity186MiniGameQuestion>,
    by_id: HashMap<i32, usize>,
}

impl Actvity186MiniGameQuestionTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Actvity186MiniGameQuestion> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Actvity186MiniGameQuestion> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Actvity186MiniGameQuestion] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Actvity186MiniGameQuestion> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
