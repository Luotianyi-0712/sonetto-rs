// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerAssistTalent {
    #[serde(rename = "bossId")]
    pub boss_id: i32,
    #[serde(rename = "bossPassiveSkills")]
    pub boss_passive_skills: String,
    #[serde(rename = "consume")]
    pub consume: i32,
    #[serde(rename = "extraRule")]
    pub extra_rule: String,
    #[serde(rename = "heroPassiveSkills")]
    pub hero_passive_skills: String,
    #[serde(rename = "isBigNode")]
    pub is_big_node: i32,
    #[serde(rename = "nodeDesc")]
    pub node_desc: String,
    #[serde(rename = "nodeGroup")]
    pub node_group: i32,
    #[serde(rename = "nodeId")]
    pub node_id: i32,
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[serde(rename = "nodeType")]
    pub node_type: i32,
    #[serde(rename = "position")]
    pub position: String,
    #[serde(rename = "preNodeIds")]
    pub pre_node_ids: String,
    #[serde(rename = "startNode")]
    pub start_node: i32,
}

pub struct TowerAssistTalentTable {
    records: Vec<TowerAssistTalent>,
}

impl TowerAssistTalentTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<TowerAssistTalent> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[TowerAssistTalent] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerAssistTalent> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
