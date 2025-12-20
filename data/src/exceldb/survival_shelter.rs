// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalShelter {
    #[serde(rename = "difficultLv")]
    pub difficult_lv: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mapId")]
    pub map_id: String,
    #[serde(rename = "maxNpcNum")]
    pub max_npc_num: String,
    #[serde(rename = "npcPosition")]
    pub npc_position: String,
    #[serde(rename = "orderPosition")]
    pub order_position: String,
    #[serde(rename = "position")]
    pub position: String,
    #[serde(rename = "seasons")]
    pub seasons: String,
    #[serde(rename = "shelterChange")]
    pub shelter_change: String,
    #[serde(rename = "shelterId")]
    pub shelter_id: i32,
    #[serde(rename = "stormArea")]
    pub storm_area: i32,
    #[serde(rename = "stormCenter")]
    pub storm_center: String,
    #[serde(rename = "toward")]
    pub toward: String,
    #[serde(rename = "versions")]
    pub versions: String,
}

pub struct SurvivalShelterTable {
    records: Vec<SurvivalShelter>,
    by_id: HashMap<i32, usize>,
}

impl SurvivalShelterTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SurvivalShelter> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SurvivalShelter> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SurvivalShelter] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SurvivalShelter> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
