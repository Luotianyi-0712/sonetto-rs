// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekwalkVer2Element {
    #[serde(rename = "bonusGroup")]
    pub bonus_group: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "disappearEffect")]
    pub disappear_effect: String,
    #[serde(rename = "effect")]
    pub effect: String,
    #[serde(rename = "generalType")]
    pub general_type: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isBoss")]
    pub is_boss: i32,
    #[serde(rename = "lightOffsetPos")]
    pub light_offset_pos: String,
    #[serde(rename = "param")]
    pub param: String,
    #[serde(rename = "pos")]
    pub pos: String,
    #[serde(rename = "res")]
    pub res: String,
    #[serde(rename = "roundId")]
    pub round_id: i32,
    #[serde(rename = "skipFinish")]
    pub skip_finish: i32,
    #[serde(rename = "smokeMaskOffset")]
    pub smoke_mask_offset: String,
    #[serde(rename = "starOffsetPos")]
    pub star_offset_pos: String,
    #[serde(rename = "tipOffsetPos")]
    pub tip_offset_pos: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

pub struct WeekwalkVer2ElementTable {
    records: Vec<WeekwalkVer2Element>,
    by_id: HashMap<i32, usize>,
}

impl WeekwalkVer2ElementTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<WeekwalkVer2Element> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&WeekwalkVer2Element> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[WeekwalkVer2Element] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, WeekwalkVer2Element> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
