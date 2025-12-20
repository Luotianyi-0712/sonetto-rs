// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomBuilding {
    #[serde(rename = "alphaThreshold")]
    pub alpha_threshold: i32,
    #[serde(rename = "areaId")]
    pub area_id: i32,
    #[serde(rename = "audioExtendIds")]
    pub audio_extend_ids: String,
    #[serde(rename = "audioExtendType")]
    pub audio_extend_type: i32,
    #[serde(rename = "buildDegree")]
    pub build_degree: i32,
    #[serde(rename = "buildingShowType")]
    pub building_show_type: i32,
    #[serde(rename = "buildingType")]
    pub building_type: i32,
    #[serde(rename = "canExchange")]
    pub can_exchange: bool,
    #[serde(rename = "canLevelUp")]
    pub can_level_up: bool,
    #[serde(rename = "canPlaceBlock")]
    pub can_place_block: String,
    #[serde(rename = "center")]
    pub center: String,
    #[serde(rename = "costResource")]
    pub cost_resource: String,
    #[serde(rename = "crossload")]
    pub crossload: i32,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "dragUpHeight")]
    pub drag_up_height: i32,
    #[serde(rename = "gatherDesc")]
    pub gather_desc: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isAreaMainBuilding")]
    pub is_area_main_building: bool,
    #[serde(rename = "linkBlock")]
    pub link_block: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "numLimit")]
    pub num_limit: i32,
    #[serde(rename = "offset")]
    pub offset: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "placeAudio")]
    pub place_audio: i32,
    #[serde(rename = "produceDesc")]
    pub produce_desc: String,
    #[serde(rename = "rare")]
    pub rare: i32,
    #[serde(rename = "reflerction")]
    pub reflerction: i32,
    #[serde(rename = "replaceBlock")]
    pub replace_block: String,
    #[serde(rename = "rewardIcon")]
    pub reward_icon: String,
    #[serde(rename = "rotate")]
    pub rotate: i32,
    #[serde(rename = "sound")]
    pub sound: i32,
    #[serde(rename = "sources")]
    pub sources: String,
    #[serde(rename = "sourcesType")]
    pub sources_type: String,
    #[serde(rename = "uiScale")]
    pub ui_scale: i32,
    #[serde(rename = "useDesc")]
    pub use_desc: String,
    #[serde(rename = "vehicleId")]
    pub vehicle_id: i32,
    #[serde(rename = "vehicleType")]
    pub vehicle_type: i32,
}

pub struct RoomBuildingTable {
    records: Vec<RoomBuilding>,
    by_id: HashMap<i32, usize>,
}

impl RoomBuildingTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RoomBuilding> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&RoomBuilding> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[RoomBuilding] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomBuilding> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
