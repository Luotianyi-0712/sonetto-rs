// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity128Stage {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "bossRushLevelDetailFullBgSimage")]
    pub boss_rush_level_detail_full_bg_simage: String,
    #[serde(rename = "bossRushMainItemBossSprite")]
    pub boss_rush_main_item_boss_sprite: String,
    #[serde(rename = "layer4MaxPoints")]
    pub layer4_max_points: i32,
    #[serde(rename = "maxPoints")]
    pub max_points: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "name_en")]
    pub name_en: String,
    #[serde(rename = "openDay")]
    pub open_day: i32,
    #[serde(rename = "resultViewFullBgSImage")]
    pub result_view_full_bg_s_image: String,
    #[serde(rename = "resultViewNameSImage")]
    pub result_view_name_s_image: String,
    #[serde(rename = "skinIds")]
    pub skin_ids: String,
    #[serde(rename = "skinOffsetXYs")]
    pub skin_offset_x_ys: String,
    #[serde(rename = "skinScales")]
    pub skin_scales: String,
    #[serde(rename = "stage")]
    pub stage: i32,
}

pub struct Activity128StageTable {
    records: Vec<Activity128Stage>,
}

impl Activity128StageTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity128Stage> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity128Stage] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity128Stage> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
