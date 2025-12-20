// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity191Role {
    #[serde(rename = "activeSkill1")]
    pub active_skill1: String,
    #[serde(rename = "activeSkill2")]
    pub active_skill2: String,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "career")]
    pub career: i32,
    #[serde(rename = "dmgType")]
    pub dmg_type: i32,
    #[serde(rename = "exLevel")]
    pub ex_level: i32,
    #[serde(rename = "facetsId")]
    pub facets_id: String,
    #[serde(rename = "gender")]
    pub gender: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "passiveSkill")]
    pub passive_skill: String,
    #[serde(rename = "powerMax")]
    pub power_max: String,
    #[serde(rename = "quality")]
    pub quality: i32,
    #[serde(rename = "roleId")]
    pub role_id: i32,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
    #[serde(rename = "star")]
    pub star: i32,
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(rename = "template")]
    pub template: i32,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "uniqueSkill")]
    pub unique_skill: i32,
    #[serde(rename = "uniqueSkill_point")]
    pub unique_skill_point: String,
    #[serde(rename = "weight")]
    pub weight: i32,
}

pub struct Activity191RoleTable {
    records: Vec<Activity191Role>,
    by_id: HashMap<i32, usize>,
}

impl Activity191RoleTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity191Role> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity191Role> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity191Role] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity191Role> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
