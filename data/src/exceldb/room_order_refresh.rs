// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomOrderRefresh {
    #[serde(rename = "finishLimitDaily")]
    pub finish_limit_daily: i32,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "meanwhileWholesaleNum")]
    pub meanwhile_wholesale_num: i32,
    #[serde(rename = "qualityWeight")]
    pub quality_weight: String,
    #[serde(rename = "wholesaleGoodsWeight")]
    pub wholesale_goods_weight: String,
    #[serde(rename = "wholesaleRevenueWeeklyLimit")]
    pub wholesale_revenue_weekly_limit: i32,
}

pub struct RoomOrderRefreshTable {
    records: Vec<RoomOrderRefresh>,
}

impl RoomOrderRefreshTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<RoomOrderRefresh> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[RoomOrderRefresh] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomOrderRefresh> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
