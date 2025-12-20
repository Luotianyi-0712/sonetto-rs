// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerAssistBoss {
    #[serde(rename = "activeSkills")]
    pub active_skills: String,
    #[serde(rename = "bossDesc")]
    pub boss_desc: String,
    #[serde(rename = "bossId")]
    pub boss_id: i32,
    #[serde(rename = "bossPic")]
    pub boss_pic: String,
    #[serde(rename = "bossShadowPic")]
    pub boss_shadow_pic: String,
    #[serde(rename = "career")]
    pub career: i32,
    #[serde(rename = "coldTime")]
    pub cold_time: i32,
    #[serde(rename = "dmgType")]
    pub dmg_type: i32,
    #[serde(rename = "gender")]
    pub gender: i32,
    #[serde(rename = "heartVariantId")]
    pub heart_variant_id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "passiveSkillName")]
    pub passive_skill_name: String,
    #[serde(rename = "passiveSkills")]
    pub passive_skills: String,
    #[serde(rename = "resInitVal")]
    pub res_init_val: i32,
    #[serde(rename = "resMaxVal")]
    pub res_max_val: i32,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(rename = "teachSkills")]
    pub teach_skills: String,
    #[serde(rename = "towerId")]
    pub tower_id: i32,
}

pub struct TowerAssistBossTable {
    records: Vec<TowerAssistBoss>,
}

impl TowerAssistBossTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<TowerAssistBoss> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[TowerAssistBoss] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerAssistBoss> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
