// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalNpc {
    #[serde(rename = "behavior")]
    pub behavior: i32,
    #[serde(rename = "choiceText")]
    pub choice_text: String,
    #[serde(rename = "copyIds")]
    pub copy_ids: String,
    #[serde(rename = "cost")]
    pub cost: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "extendCost")]
    pub extend_cost: i32,
    #[serde(rename = "headIcon")]
    pub head_icon: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "incidental")]
    pub incidental: String,
    #[serde(rename = "incidentalRange")]
    pub incidental_range: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "npcDesc")]
    pub npc_desc: String,
    #[serde(rename = "priority")]
    pub priority: i32,
    #[serde(rename = "rare")]
    pub rare: i32,
    #[serde(rename = "recruitment")]
    pub recruitment: i32,
    #[serde(rename = "renown")]
    pub renown: String,
    #[serde(rename = "resource")]
    pub resource: String,
    #[serde(rename = "roll")]
    pub roll: i32,
    #[serde(rename = "rotate")]
    pub rotate: String,
    #[serde(rename = "seasons")]
    pub seasons: String,
    #[serde(rename = "smallIcon")]
    pub small_icon: String,
    #[serde(rename = "subType")]
    pub sub_type: i32,
    #[serde(rename = "surBehavior")]
    pub sur_behavior: String,
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(rename = "takeOut")]
    pub take_out: i32,
    #[serde(rename = "transferId")]
    pub transfer_id: String,
    #[serde(rename = "versions")]
    pub versions: String,
    #[serde(rename = "waterResource")]
    pub water_resource: String,
    #[serde(rename = "worldLevel")]
    pub world_level: String,
}

pub struct SurvivalNpcTable {
    records: Vec<SurvivalNpc>,
    by_id: HashMap<i32, usize>,
}

impl SurvivalNpcTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<SurvivalNpc> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SurvivalNpc> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SurvivalNpc] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SurvivalNpc> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
