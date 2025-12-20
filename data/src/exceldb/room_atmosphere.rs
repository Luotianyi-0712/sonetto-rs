// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomAtmosphere {
    #[serde(rename = "buildingId")]
    pub building_id: i32,
    #[serde(rename = "cdtimes")]
    pub cdtimes: i32,
    #[serde(rename = "cyclesTimes")]
    pub cycles_times: i32,
    #[serde(rename = "durationDay")]
    pub duration_day: i32,
    #[serde(rename = "effectSequence")]
    pub effect_sequence: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "openTime")]
    pub open_time: String,
    #[serde(rename = "residentEffect")]
    pub resident_effect: i32,
    #[serde(rename = "triggerType")]
    pub trigger_type: i32,
}

pub struct RoomAtmosphereTable {
    records: Vec<RoomAtmosphere>,
    by_id: HashMap<i32, usize>,
}

impl RoomAtmosphereTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RoomAtmosphere> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&RoomAtmosphere> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[RoomAtmosphere] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomAtmosphere> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
