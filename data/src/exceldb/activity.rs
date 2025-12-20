// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    #[serde(rename = "achievementGroup")]
    pub achievement_group: i32,
    #[serde(rename = "achievementGroupPath")]
    pub achievement_group_path: String,
    #[serde(rename = "achievementJumpId")]
    pub achievement_jump_id: i32,
    #[serde(rename = "actDesc")]
    pub act_desc: String,
    #[serde(rename = "actTip")]
    pub act_tip: String,
    #[serde(rename = "activityBonus")]
    pub activity_bonus: String,
    #[serde(rename = "banner")]
    pub banner: String,
    #[serde(rename = "confirmCondition")]
    pub confirm_condition: String,
    #[serde(rename = "defaultPriority")]
    pub default_priority: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "displayPriority")]
    pub display_priority: i32,
    #[serde(rename = "extraDisplayIcon")]
    pub extra_display_icon: String,
    #[serde(rename = "extraDisplayId")]
    pub extra_display_id: i32,
    #[serde(rename = "hintPriority")]
    pub hint_priority: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isRetroAcitivity")]
    pub is_retro_acitivity: i32,
    #[serde(rename = "joinCondition")]
    pub join_condition: String,
    #[serde(rename = "logoName")]
    pub logo_name: String,
    #[serde(rename = "logoType")]
    pub logo_type: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "openId")]
    pub open_id: i32,
    #[serde(rename = "param")]
    pub param: String,
    #[serde(rename = "permanentParentAcitivityId")]
    pub permanent_parent_acitivity_id: i32,
    #[serde(rename = "redDotId")]
    pub red_dot_id: i32,
    #[serde(rename = "showCenter")]
    pub show_center: i32,
    #[serde(rename = "storyId")]
    pub story_id: i32,
    #[serde(rename = "tabBgPath")]
    pub tab_bg_path: String,
    #[serde(rename = "tabBgmId")]
    pub tab_bgm_id: i32,
    #[serde(rename = "tabButton")]
    pub tab_button: String,
    #[serde(rename = "tabName")]
    pub tab_name: String,
    #[serde(rename = "tryoutEpisode")]
    pub tryout_episode: i32,
    #[serde(rename = "tryoutcharacter")]
    pub tryoutcharacter: i32,
    #[serde(rename = "typeId")]
    pub type_id: i32,
}

pub struct ActivityTable {
    records: Vec<Activity>,
    by_id: HashMap<i32, usize>,
}

impl ActivityTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Activity> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
