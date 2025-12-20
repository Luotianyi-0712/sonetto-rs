// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreChargeGoods {
    #[serde(rename = "belongStoreId")]
    pub belong_store_id: i32,
    #[serde(rename = "bigImg")]
    pub big_img: String,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    #[serde(rename = "currencyCodejp")]
    pub currency_codejp: String,
    #[serde(rename = "currencyCodekr")]
    pub currency_codekr: String,
    #[serde(rename = "currencyCodetw")]
    pub currency_codetw: String,
    #[serde(rename = "currencyCodezh")]
    pub currency_codezh: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "detailDesc")]
    pub detail_desc: String,
    #[serde(rename = "diamond")]
    pub diamond: i32,
    #[serde(rename = "extraDiamond")]
    pub extra_diamond: i32,
    #[serde(rename = "firstDiamond")]
    pub first_diamond: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isOnline")]
    pub is_online: bool,
    #[serde(rename = "item")]
    pub item: String,
    #[serde(rename = "limit")]
    pub limit: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "newEndTime")]
    pub new_end_time: String,
    #[serde(rename = "newStartTime")]
    pub new_start_time: String,
    #[serde(rename = "notShowInRecommend")]
    pub not_show_in_recommend: bool,
    #[serde(rename = "offTag")]
    pub off_tag: String,
    #[serde(rename = "offlineTime")]
    pub offline_time: String,
    #[serde(rename = "onlineTime")]
    pub online_time: String,
    #[serde(rename = "order")]
    pub order: i32,
    #[serde(rename = "originalCost")]
    pub original_cost: f32,
    #[serde(rename = "originalCostGoodsId")]
    pub original_cost_goods_id: i32,
    #[serde(rename = "originalCostjp")]
    pub original_costjp: f32,
    #[serde(rename = "originalCostkr")]
    pub original_costkr: f32,
    #[serde(rename = "originalCosttw")]
    pub original_costtw: f32,
    #[serde(rename = "originalCostzh")]
    pub original_costzh: f32,
    #[serde(rename = "overviewJumpId")]
    pub overview_jump_id: String,
    #[serde(rename = "preGoodsId")]
    pub pre_goods_id: i32,
    #[serde(rename = "price")]
    pub price: f32,
    #[serde(rename = "pricejp")]
    pub pricejp: f32,
    #[serde(rename = "pricekr")]
    pub pricekr: f32,
    #[serde(rename = "pricetw")]
    pub pricetw: f32,
    #[serde(rename = "pricezh")]
    pub pricezh: f32,
    #[serde(rename = "product")]
    pub product: String,
    #[serde(rename = "quickUseItemList")]
    pub quick_use_item_list: String,
    #[serde(rename = "showLinkageTag")]
    pub show_linkage_tag: bool,
    #[serde(rename = "showLogoTag")]
    pub show_logo_tag: bool,
    #[serde(rename = "taskid")]
    pub taskid: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct StoreChargeGoodsTable {
    records: Vec<StoreChargeGoods>,
    by_id: HashMap<i32, usize>,
}

impl StoreChargeGoodsTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        
        // Parse the [table_name, [records]] format
        let value: serde_json::Value = serde_json::from_str(&json)?;
        let records: Vec<StoreChargeGoods> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&StoreChargeGoods> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[StoreChargeGoods] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, StoreChargeGoods> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
