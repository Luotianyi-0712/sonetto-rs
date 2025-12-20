// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeEpisode {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "condition")]
    pub condition: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "hiddenRule1")]
    pub hidden_rule1: String,
    #[serde(rename = "hiddenRule2")]
    pub hidden_rule2: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "leaderBaseStress")]
    pub leader_base_stress: i32,
    #[serde(rename = "leaderIdentity")]
    pub leader_identity: String,
    #[serde(rename = "leaderMaxStress")]
    pub leader_max_stress: i32,
    #[serde(rename = "leaderPosition")]
    pub leader_position: i32,
    #[serde(rename = "leaderSkill")]
    pub leader_skill: String,
    #[serde(rename = "order")]
    pub order: i32,
    #[serde(rename = "preEpisodeIds")]
    pub pre_episode_ids: String,
    #[serde(rename = "reportIcon")]
    pub report_icon: String,
    #[serde(rename = "resultIcon")]
    pub result_icon: String,
    #[serde(rename = "rule1")]
    pub rule1: String,
    #[serde(rename = "rule2")]
    pub rule2: String,
    #[serde(rename = "ruleDesc1")]
    pub rule_desc1: String,
    #[serde(rename = "ruleDesc2")]
    pub rule_desc2: String,
    #[serde(rename = "ruleIcon")]
    pub rule_icon: String,
    #[serde(rename = "skillDesc")]
    pub skill_desc: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct ChallengeEpisodeTable {
    records: Vec<ChallengeEpisode>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl ChallengeEpisodeTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<ChallengeEpisode> = if let Some(array) = value.as_array() {
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
                by_group.entry(record.group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_group,
        })
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &'_ ChallengeEpisode> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[ChallengeEpisode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, ChallengeEpisode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
