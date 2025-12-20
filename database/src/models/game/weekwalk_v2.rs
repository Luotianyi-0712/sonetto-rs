use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct UserWeekwalkV2Info {
    pub user_id: i64,
    pub time_id: i32,
    pub start_time: i64,
    pub end_time: i64,
    pub pop_rule: bool,
}

#[derive(Debug, Clone, FromRow)]
pub struct WeekwalkV2Layer {
    pub layer_id: i32,
    pub scene_id: i32,
    pub all_pass: bool,
    pub finished: bool,
    pub unlock: bool,
    pub show_finished: bool,
    pub params: String,
}

#[derive(Debug, Clone)]
pub struct WeekwalkV2CupInfo {
    pub cup_id: i32,
    pub result: i32,
}

impl From<WeekwalkV2CupInfo> for sonettobuf::WeekwalkVer2CupInfo {
    fn from(c: WeekwalkV2CupInfo) -> Self {
        sonettobuf::WeekwalkVer2CupInfo {
            id: Some(c.cup_id),
            result: Some(c.result),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WeekwalkV2ElementInfo {
    pub element_id: i32,
    pub finish: bool,
    pub index_num: i32,
    pub visible: bool,
}

impl From<WeekwalkV2ElementInfo> for sonettobuf::WeekwalkVer2ElementInfo {
    fn from(e: WeekwalkV2ElementInfo) -> Self {
        sonettobuf::WeekwalkVer2ElementInfo {
            element_id: Some(e.element_id),
            finish: Some(e.finish),
            index: Some(e.index_num),
            visible: Some(e.visible),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct WeekwalkV2PrevSettle {
    pub user_id: i64,
    pub max_layer_id: i32,
    pub max_battle_id: i32,
    pub max_battle_index: i32,
    pub show: bool,
}

#[derive(Debug, Clone)]
pub struct WeekwalkV2PrevSettleLayerInfo {
    pub layer_id: i32,
    pub platinum_cup_num: i32,
}

impl From<WeekwalkV2PrevSettleLayerInfo> for sonettobuf::WeekwalkVer2PrevSettleLayerInfo {
    fn from(l: WeekwalkV2PrevSettleLayerInfo) -> Self {
        sonettobuf::WeekwalkVer2PrevSettleLayerInfo {
            layer_id: Some(l.layer_id),
            platinum_cup_num: Some(l.platinum_cup_num),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WeekwalkV2SnapshotInfo {
    pub snapshot_no: i32,
    pub skill_ids: Vec<i32>,
}

impl From<WeekwalkV2SnapshotInfo> for sonettobuf::WeekwalkVer2SnapshotInfo {
    fn from(s: WeekwalkV2SnapshotInfo) -> Self {
        sonettobuf::WeekwalkVer2SnapshotInfo {
            no: Some(s.snapshot_no),
            skill_ids: s.skill_ids,
        }
    }
}
