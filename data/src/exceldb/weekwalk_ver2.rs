// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekwalkVer2 {
    #[serde(rename = "chooseSkillNum")]
    pub choose_skill_num: i32,
    #[serde(rename = "fightIdFront")]
    pub fight_id_front: i32,
    #[serde(rename = "fightIdRear")]
    pub fight_id_rear: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "issueId")]
    pub issue_id: i32,
    #[serde(rename = "layer")]
    pub layer: i32,
    #[serde(rename = "preId")]
    pub pre_id: i32,
    #[serde(rename = "resIdFront")]
    pub res_id_front: i32,
    #[serde(rename = "resIdRear")]
    pub res_id_rear: i32,
    #[serde(rename = "sceneId")]
    pub scene_id: i32,
}

pub struct WeekwalkVer2Table {
    records: Vec<WeekwalkVer2>,
    by_id: HashMap<i32, usize>,
}

impl WeekwalkVer2Table {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<WeekwalkVer2> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&WeekwalkVer2> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[WeekwalkVer2] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, WeekwalkVer2> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
