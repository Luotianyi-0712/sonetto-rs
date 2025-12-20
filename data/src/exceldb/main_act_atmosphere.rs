// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainActAtmosphere {
    #[serde(rename = "effectDuration")]
    pub effect_duration: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isShowActBg")]
    pub is_show_act_bg: bool,
    #[serde(rename = "isShowLogo")]
    pub is_show_logo: bool,
    #[serde(rename = "isShowfx")]
    pub is_showfx: bool,
    #[serde(rename = "mainThumbnailView")]
    pub main_thumbnail_view: Option<serde_json::Value>,
    #[serde(rename = "mainThumbnailViewActBg")]
    pub main_thumbnail_view_act_bg: bool,
    #[serde(rename = "mainView")]
    pub main_view: Option<serde_json::Value>,
    #[serde(rename = "mainViewActBtn")]
    pub main_view_act_btn: Option<serde_json::Value>,
    #[serde(rename = "mainViewActBtnPrefix")]
    pub main_view_act_btn_prefix: String,
}

pub struct MainActAtmosphereTable {
    records: Vec<MainActAtmosphere>,
    by_id: HashMap<i32, usize>,
}

impl MainActAtmosphereTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<MainActAtmosphere> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&MainActAtmosphere> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[MainActAtmosphere] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, MainActAtmosphere> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
