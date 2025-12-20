// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity148SkillType {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "skillIcon")]
    pub skill_icon: String,
    #[serde(rename = "skillInfoDesc")]
    pub skill_info_desc: String,
    #[serde(rename = "skillName")]
    pub skill_name: String,
    #[serde(rename = "skillNameEn")]
    pub skill_name_en: String,
    #[serde(rename = "skillValueDesc")]
    pub skill_value_desc: String,
}

pub struct Activity148SkillTypeTable {
    records: Vec<Activity148SkillType>,
    by_id: HashMap<i32, usize>,
}

impl Activity148SkillTypeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity148SkillType> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity148SkillType> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity148SkillType] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity148SkillType> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
