// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterSkin {
    #[serde(rename = "bossSkillSpeed")]
    pub boss_skill_speed: i32,
    #[serde(rename = "canHide")]
    pub can_hide: i32,
    #[serde(rename = "clickBoxUnlimit")]
    pub click_box_unlimit: i32,
    #[serde(rename = "colorbg")]
    pub colorbg: i32,
    #[serde(rename = "des")]
    pub des: String,
    #[serde(rename = "effect")]
    pub effect: String,
    #[serde(rename = "effectHangPoint")]
    pub effect_hang_point: String,
    #[serde(rename = "fight_special")]
    pub fight_special: i32,
    #[serde(rename = "flipX")]
    pub flip_x: i32,
    #[serde(rename = "floatOffset")]
    pub float_offset: String,
    #[serde(rename = "focusOffset")]
    pub focus_offset: Option<serde_json::Value>,
    #[serde(rename = "headIcon")]
    pub head_icon: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isFly")]
    pub is_fly: i32,
    #[serde(rename = "mainBody")]
    pub main_body: i32,
    #[serde(rename = "matId")]
    pub mat_id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nameEng")]
    pub name_eng: String,
    #[serde(rename = "noDeadEffect")]
    pub no_dead_effect: i32,
    #[serde(rename = "outlineWidth")]
    pub outline_width: f32,
    #[serde(rename = "retangleIcon")]
    pub retangle_icon: String,
    #[serde(rename = "showTemplate")]
    pub show_template: i32,
    #[serde(rename = "skills")]
    pub skills: String,
    #[serde(rename = "spine")]
    pub spine: String,
    #[serde(rename = "topuiOffset")]
    pub topui_offset: Option<serde_json::Value>,
    #[serde(rename = "weatherParam")]
    pub weather_param: i32,
}

pub struct MonsterSkinTable {
    records: Vec<MonsterSkin>,
    by_id: HashMap<i32, usize>,
}

impl MonsterSkinTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<MonsterSkin> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&MonsterSkin> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[MonsterSkin] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, MonsterSkin> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
