// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Turnback {
    #[serde(rename = "additionChapterIds")]
    pub addition_chapter_ids: String,
    #[serde(rename = "additionDurationDays")]
    pub addition_duration_days: i32,
    #[serde(rename = "additionRate")]
    pub addition_rate: i32,
    #[serde(rename = "additionType")]
    pub addition_type: String,
    #[serde(rename = "bindActivityId")]
    pub bind_activity_id: i32,
    #[serde(rename = "bonusList")]
    pub bonus_list: String,
    #[serde(rename = "bonusPointMaterial")]
    pub bonus_point_material: String,
    #[serde(rename = "buyDoubleBonusPrice")]
    pub buy_double_bonus_price: String,
    #[serde(rename = "canBuyDoubleBonus")]
    pub can_buy_double_bonus: bool,
    #[serde(rename = "condition")]
    pub condition: String,
    #[serde(rename = "durationDays")]
    pub duration_days: i32,
    #[serde(rename = "endStory")]
    pub end_story: i32,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "jumpId")]
    pub jump_id: i32,
    #[serde(rename = "monthCardAddedId")]
    pub month_card_added_id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "offlineDays")]
    pub offline_days: i32,
    #[serde(rename = "onceBonus")]
    pub once_bonus: String,
    #[serde(rename = "onlineDurationDays")]
    pub online_duration_days: i32,
    #[serde(rename = "priority")]
    pub priority: i32,
    #[serde(rename = "startStory")]
    pub start_story: i32,
    #[serde(rename = "subModuleIds")]
    pub sub_module_ids: String,
    #[serde(rename = "taskBonusMailId")]
    pub task_bonus_mail_id: i32,
}

pub struct TurnbackTable {
    records: Vec<Turnback>,
    by_id: HashMap<i32, usize>,
}

impl TurnbackTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Turnback> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Turnback> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Turnback] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Turnback> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
