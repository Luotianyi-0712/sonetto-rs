// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity157FactoryComponent {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "bonus")]
    pub bonus: String,
    #[serde(rename = "bonusBuildingLevel")]
    pub bonus_building_level: String,
    #[serde(rename = "bonusForShow")]
    pub bonus_for_show: String,
    #[serde(rename = "componentId")]
    pub component_id: i32,
    #[serde(rename = "elementId")]
    pub element_id: i32,
    #[serde(rename = "nextComponentId")]
    pub next_component_id: i32,
    #[serde(rename = "preComponentId")]
    pub pre_component_id: i32,
    #[serde(rename = "unlockCondition")]
    pub unlock_condition: String,
}

pub struct Activity157FactoryComponentTable {
    records: Vec<Activity157FactoryComponent>,
}

impl Activity157FactoryComponentTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<Activity157FactoryComponent> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity157FactoryComponent] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity157FactoryComponent> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
