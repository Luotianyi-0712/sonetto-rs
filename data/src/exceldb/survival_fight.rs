// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalFight {
    #[serde(rename = "battleId")]
    pub battle_id: i32,
    #[serde(rename = "behavior")]
    pub behavior: i32,
    #[serde(rename = "camera")]
    pub camera: i32,
    #[serde(rename = "choiceText")]
    pub choice_text: String,
    #[serde(rename = "copyIds")]
    pub copy_ids: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "enforce")]
    pub enforce: i32,
    #[serde(rename = "fightLevel")]
    pub fight_level: i32,
    #[serde(rename = "grid")]
    pub grid: String,
    #[serde(rename = "group")]
    pub group: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "incidental")]
    pub incidental: String,
    #[serde(rename = "incidentalRange")]
    pub incidental_range: String,
    #[serde(rename = "isRepeat")]
    pub is_repeat: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "priority")]
    pub priority: i32,
    #[serde(rename = "resource")]
    pub resource: String,
    #[serde(rename = "roll")]
    pub roll: i32,
    #[serde(rename = "rotate")]
    pub rotate: String,
    #[serde(rename = "seasons")]
    pub seasons: String,
    #[serde(rename = "skip")]
    pub skip: i32,
    #[serde(rename = "subType")]
    pub sub_type: i32,
    #[serde(rename = "transferId")]
    pub transfer_id: String,
    #[serde(rename = "versions")]
    pub versions: String,
    #[serde(rename = "warningRange")]
    pub warning_range: i32,
    #[serde(rename = "waterResource")]
    pub water_resource: String,
    #[serde(rename = "worldLevel")]
    pub world_level: String,
}

pub struct SurvivalFightTable {
    records: Vec<SurvivalFight>,
    by_id: HashMap<i32, usize>,
}

impl SurvivalFightTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SurvivalFight> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SurvivalFight> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SurvivalFight] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SurvivalFight> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
