// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomCharacterDialog {
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "critteremoji")]
    pub critteremoji: i32,
    #[serde(rename = "dialogId")]
    pub dialog_id: i32,
    #[serde(rename = "nextStepId")]
    pub next_step_id: String,
    #[serde(rename = "relateContent")]
    pub relate_content: String,
    #[serde(rename = "selectIds")]
    pub select_ids: String,
    #[serde(rename = "speaker")]
    pub speaker: String,
    #[serde(rename = "speakerType")]
    pub speaker_type: i32,
    #[serde(rename = "stepId")]
    pub step_id: i32,
}

pub struct RoomCharacterDialogTable {
    records: Vec<RoomCharacterDialog>,
}

impl RoomCharacterDialogTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RoomCharacterDialog> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[RoomCharacterDialog] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomCharacterDialog> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
