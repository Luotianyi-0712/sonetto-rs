// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterJob {
    #[serde(rename = "addDmg_equip_super")]
    pub add_dmg_equip_super: i32,
    #[serde(rename = "addDmg_init_super")]
    pub add_dmg_init_super: i32,
    #[serde(rename = "addDmg_reson_super")]
    pub add_dmg_reson_super: i32,
    #[serde(rename = "attack_base")]
    pub attack_base: i32,
    #[serde(rename = "attack_base_coef")]
    pub attack_base_coef: i32,
    #[serde(rename = "attack_equip_base")]
    pub attack_equip_base: i32,
    #[serde(rename = "criDef_equip_super")]
    pub cri_def_equip_super: i32,
    #[serde(rename = "criDef_init_super")]
    pub cri_def_init_super: i32,
    #[serde(rename = "criDef_reson_super")]
    pub cri_def_reson_super: i32,
    #[serde(rename = "criDmg_equip_super")]
    pub cri_dmg_equip_super: i32,
    #[serde(rename = "criDmg_init_super")]
    pub cri_dmg_init_super: i32,
    #[serde(rename = "criDmg_reson_super")]
    pub cri_dmg_reson_super: i32,
    #[serde(rename = "cri_equip_super")]
    pub cri_equip_super: i32,
    #[serde(rename = "cri_init_super")]
    pub cri_init_super: i32,
    #[serde(rename = "cri_reson_super")]
    pub cri_reson_super: i32,
    #[serde(rename = "defense_base")]
    pub defense_base: i32,
    #[serde(rename = "defense_base_coef")]
    pub defense_base_coef: i32,
    #[serde(rename = "defense_equip_base")]
    pub defense_equip_base: i32,
    #[serde(rename = "dropDmg_equip_super")]
    pub drop_dmg_equip_super: i32,
    #[serde(rename = "dropDmg_init_super")]
    pub drop_dmg_init_super: i32,
    #[serde(rename = "dropDmg_reson_super")]
    pub drop_dmg_reson_super: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "life_base")]
    pub life_base: i32,
    #[serde(rename = "life_base_coef")]
    pub life_base_coef: i32,
    #[serde(rename = "life_equip_base")]
    pub life_equip_base: i32,
    #[serde(rename = "mdefense_base")]
    pub mdefense_base: i32,
    #[serde(rename = "mdefense_base_coef")]
    pub mdefense_base_coef: i32,
    #[serde(rename = "mdefense_equip_base")]
    pub mdefense_equip_base: i32,
    #[serde(rename = "recri_equip_super")]
    pub recri_equip_super: i32,
    #[serde(rename = "recri_init_super")]
    pub recri_init_super: i32,
    #[serde(rename = "recri_reson_super")]
    pub recri_reson_super: i32,
    #[serde(rename = "technic_base")]
    pub technic_base: i32,
    #[serde(rename = "technic_base_coef")]
    pub technic_base_coef: i32,
    #[serde(rename = "technic_equip_base")]
    pub technic_equip_base: i32,
}

pub struct MonsterJobTable {
    records: Vec<MonsterJob>,
    by_id: HashMap<i32, usize>,
}

impl MonsterJobTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<MonsterJob> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&MonsterJob> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[MonsterJob] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, MonsterJob> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
