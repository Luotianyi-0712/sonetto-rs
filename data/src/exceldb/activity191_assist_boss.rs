// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity191AssistBoss {
    #[serde(rename = "activeSkill1")]
    pub active_skill1: String,
    #[serde(rename = "activeSkill2")]
    pub active_skill2: String,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "bossDesc")]
    pub boss_desc: String,
    #[serde(rename = "bossId")]
    pub boss_id: i32,
    #[serde(rename = "career")]
    pub career: i32,
    #[serde(rename = "condition")]
    pub condition: String,
    #[serde(rename = "dmgType")]
    pub dmg_type: i32,
    #[serde(rename = "gender")]
    pub gender: i32,
    #[serde(rename = "headIcon")]
    pub head_icon: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "offset")]
    pub offset: String,
    #[serde(rename = "passiveSkills")]
    pub passive_skills: String,
    #[serde(rename = "powerMax")]
    pub power_max: String,
    #[serde(rename = "relation")]
    pub relation: String,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
    #[serde(rename = "uiForm")]
    pub ui_form: i32,
    #[serde(rename = "uniqueSkill")]
    pub unique_skill: i32,
}

pub struct Activity191AssistBossTable {
    records: Vec<Activity191AssistBoss>,
}

impl Activity191AssistBossTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity191AssistBoss> = if let Some(array) = value.as_array() {
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
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity191AssistBoss] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity191AssistBoss> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
