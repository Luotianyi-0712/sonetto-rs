// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekwalkVer2Time {
    #[serde(rename = "issueId")]
    pub issue_id: i32,
    #[serde(rename = "optionalSkills")]
    pub optional_skills: String,
    #[serde(rename = "ruleFront")]
    pub rule_front: String,
    #[serde(rename = "ruleIcon")]
    pub rule_icon: String,
    #[serde(rename = "ruleRear")]
    pub rule_rear: String,
    #[serde(rename = "timeId")]
    pub time_id: i32,
}

pub struct WeekwalkVer2TimeTable {
    records: Vec<WeekwalkVer2Time>,
}

impl WeekwalkVer2TimeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<WeekwalkVer2Time> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[WeekwalkVer2Time] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, WeekwalkVer2Time> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
