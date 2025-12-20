// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hero3124SkillTalent {
    #[serde(rename = "additionalFieldDesc")]
    pub additional_field_desc: String,
    #[serde(rename = "additionalFieldDesc1")]
    pub additional_field_desc1: String,
    #[serde(rename = "additionalFieldDesc2")]
    pub additional_field_desc2: String,
    #[serde(rename = "additionalFieldDesc3")]
    pub additional_field_desc3: String,
    #[serde(rename = "additionalFieldDesc4")]
    pub additional_field_desc4: String,
    #[serde(rename = "additionalFieldDesc5")]
    pub additional_field_desc5: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "desc1")]
    pub desc1: String,
    #[serde(rename = "desc2")]
    pub desc2: String,
    #[serde(rename = "desc3")]
    pub desc3: String,
    #[serde(rename = "desc4")]
    pub desc4: String,
    #[serde(rename = "desc5")]
    pub desc5: String,
    #[serde(rename = "exchangeSkills0")]
    pub exchange_skills0: String,
    #[serde(rename = "exchangeSkills1")]
    pub exchange_skills1: String,
    #[serde(rename = "exchangeSkills2")]
    pub exchange_skills2: String,
    #[serde(rename = "exchangeSkills3")]
    pub exchange_skills3: String,
    #[serde(rename = "exchangeSkills4")]
    pub exchange_skills4: String,
    #[serde(rename = "exchangeSkills5")]
    pub exchange_skills5: String,
    #[serde(rename = "fieldActivateDesc")]
    pub field_activate_desc: String,
    #[serde(rename = "fieldActivateDesc1")]
    pub field_activate_desc1: String,
    #[serde(rename = "fieldActivateDesc2")]
    pub field_activate_desc2: String,
    #[serde(rename = "fieldActivateDesc3")]
    pub field_activate_desc3: String,
    #[serde(rename = "fieldActivateDesc4")]
    pub field_activate_desc4: String,
    #[serde(rename = "fieldActivateDesc5")]
    pub field_activate_desc5: String,
    #[serde(rename = "fieldDesc")]
    pub field_desc: String,
    #[serde(rename = "fieldDesc1")]
    pub field_desc1: String,
    #[serde(rename = "fieldDesc2")]
    pub field_desc2: String,
    #[serde(rename = "fieldDesc3")]
    pub field_desc3: String,
    #[serde(rename = "fieldDesc4")]
    pub field_desc4: String,
    #[serde(rename = "fieldDesc5")]
    pub field_desc5: String,
    #[serde(rename = "fieldName")]
    pub field_name: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "newSkills0")]
    pub new_skills0: String,
    #[serde(rename = "newSkills1")]
    pub new_skills1: String,
    #[serde(rename = "newSkills2")]
    pub new_skills2: String,
    #[serde(rename = "newSkills3")]
    pub new_skills3: String,
    #[serde(rename = "newSkills4")]
    pub new_skills4: String,
    #[serde(rename = "newSkills5")]
    pub new_skills5: String,
    #[serde(rename = "sub")]
    pub sub: i32,
    #[serde(rename = "talentId")]
    pub talent_id: i32,
}

pub struct Hero3124SkillTalentTable {
    records: Vec<Hero3124SkillTalent>,
}

impl Hero3124SkillTalentTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Hero3124SkillTalent> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Hero3124SkillTalent] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Hero3124SkillTalent> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
