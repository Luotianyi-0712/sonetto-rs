// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mail {
    #[serde(rename = "addressee")]
    pub addressee: String,
    #[serde(rename = "attachment")]
    pub attachment: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "copy")]
    pub copy: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "jump")]
    pub jump: String,
    #[serde(rename = "jumpTitle")]
    pub jump_title: String,
    #[serde(rename = "needShowToast")]
    pub need_show_toast: i32,
    #[serde(rename = "sender")]
    pub sender: String,
    #[serde(rename = "senderType")]
    pub sender_type: i32,
    #[serde(rename = "specialTag")]
    pub special_tag: i32,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct MailTable {
    records: Vec<Mail>,
    by_id: HashMap<i32, usize>,
}

impl MailTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Mail> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Mail> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Mail] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Mail> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
