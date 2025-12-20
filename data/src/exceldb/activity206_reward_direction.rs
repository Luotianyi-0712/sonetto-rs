// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity206RewardDirection {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "baseRate")]
    pub base_rate: i32,
    #[serde(rename = "des")]
    pub des: String,
    #[serde(rename = "directionId")]
    pub direction_id: i32,
    #[serde(rename = "guaranteeCount")]
    pub guarantee_count: i32,
    #[serde(rename = "haveGuarantee")]
    pub have_guarantee: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nextDayDecRate")]
    pub next_day_dec_rate: i32,
    #[serde(rename = "pic")]
    pub pic: String,
    #[serde(rename = "rewardGroupId")]
    pub reward_group_id: i32,
}

pub struct Activity206RewardDirectionTable {
    records: Vec<Activity206RewardDirection>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl Activity206RewardDirectionTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity206RewardDirection> = if let Some(array) = value.as_array() {
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
        
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
                by_group.entry(record.reward_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_group,
        })
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &'_ Activity206RewardDirection> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Activity206RewardDirection] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity206RewardDirection> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
