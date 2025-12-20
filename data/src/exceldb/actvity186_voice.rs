// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actvity186Voice {
    #[serde(rename = "audio")]
    pub audio: i32,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "decontent")]
    pub decontent: String,
    #[serde(rename = "deface")]
    pub deface: String,
    #[serde(rename = "demotion")]
    pub demotion: String,
    #[serde(rename = "demouth")]
    pub demouth: String,
    #[serde(rename = "displayTime")]
    pub display_time: i32,
    #[serde(rename = "encontent")]
    pub encontent: String,
    #[serde(rename = "enface")]
    pub enface: String,
    #[serde(rename = "enmotion")]
    pub enmotion: String,
    #[serde(rename = "enmouth")]
    pub enmouth: String,
    #[serde(rename = "face")]
    pub face: String,
    #[serde(rename = "frcontent")]
    pub frcontent: String,
    #[serde(rename = "frface")]
    pub frface: String,
    #[serde(rename = "frmotion")]
    pub frmotion: String,
    #[serde(rename = "frmouth")]
    pub frmouth: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "jpcontent")]
    pub jpcontent: String,
    #[serde(rename = "jpface")]
    pub jpface: String,
    #[serde(rename = "jpmotion")]
    pub jpmotion: String,
    #[serde(rename = "jpmouth")]
    pub jpmouth: String,
    #[serde(rename = "kocontent")]
    pub kocontent: String,
    #[serde(rename = "koface")]
    pub koface: String,
    #[serde(rename = "komotion")]
    pub komotion: String,
    #[serde(rename = "komouth")]
    pub komouth: String,
    #[serde(rename = "motion")]
    pub motion: String,
    #[serde(rename = "mouth")]
    pub mouth: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "param")]
    pub param: String,
    #[serde(rename = "show")]
    pub show: i32,
    #[serde(rename = "sortId")]
    pub sort_id: i32,
    #[serde(rename = "thaicontent")]
    pub thaicontent: String,
    #[serde(rename = "thaiface")]
    pub thaiface: String,
    #[serde(rename = "thaimotion")]
    pub thaimotion: String,
    #[serde(rename = "thaimouth")]
    pub thaimouth: String,
    #[serde(rename = "time")]
    pub time: String,
    #[serde(rename = "twcontent")]
    pub twcontent: String,
    #[serde(rename = "twface")]
    pub twface: String,
    #[serde(rename = "twmotion")]
    pub twmotion: String,
    #[serde(rename = "twmouth")]
    pub twmouth: String,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "unlockCondition")]
    pub unlock_condition: String,
    #[serde(rename = "viewEffect")]
    pub view_effect: i32,
}

pub struct Actvity186VoiceTable {
    records: Vec<Actvity186Voice>,
    by_id: HashMap<i32, usize>,
}

impl Actvity186VoiceTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Actvity186Voice> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Actvity186Voice> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Actvity186Voice] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Actvity186Voice> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
