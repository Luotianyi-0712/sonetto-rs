// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuideStep {
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "additionCmd")]
    pub addition_cmd: String,
    #[serde(rename = "againSteps")]
    pub again_steps: String,
    #[serde(rename = "audio")]
    pub audio: i32,
    #[serde(rename = "delay")]
    pub delay: f32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "exception")]
    pub exception: String,
    #[serde(rename = "exceptionDelay")]
    pub exception_delay: i32,
    #[serde(rename = "goPath")]
    pub go_path: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "keyStep")]
    pub key_step: i32,
    #[serde(rename = "maskId")]
    pub mask_id: i32,
    #[serde(rename = "notForce")]
    pub not_force: i32,
    #[serde(rename = "portraitPos")]
    pub portrait_pos: i32,
    #[serde(rename = "stat")]
    pub stat: i32,
    #[serde(rename = "stepId")]
    pub step_id: i32,
    #[serde(rename = "storyContent")]
    pub story_content: String,
    #[serde(rename = "tipsContent")]
    pub tips_content: String,
    #[serde(rename = "tipsDir")]
    pub tips_dir: i32,
    #[serde(rename = "tipsHead")]
    pub tips_head: String,
    #[serde(rename = "tipsPos")]
    pub tips_pos: String,
    #[serde(rename = "tipsTalker")]
    pub tips_talker: String,
    #[serde(rename = "touchGOPath")]
    pub touch_g_o_path: String,
    #[serde(rename = "uiInfo")]
    pub ui_info: String,
    #[serde(rename = "uiOffset")]
    pub ui_offset: String,
}

pub struct GuideStepTable {
    records: Vec<GuideStep>,
    by_id: HashMap<i32, usize>,
}

impl GuideStepTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<GuideStep> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&GuideStep> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[GuideStep] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, GuideStep> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
