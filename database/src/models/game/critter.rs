use sonettobuf;
use sqlx::FromRow;

#[allow(dead_code)]
#[derive(Debug, Clone, FromRow)]
pub struct CritterRecord {
    pub uid: i64,
    pub player_id: i64,
    pub define_id: i32,
    pub create_time: i64,
    pub efficiency: i32,
    pub patience: i32,
    pub lucky: i32,
    pub efficiency_incr_rate: i32,
    pub patience_incr_rate: i32,
    pub lucky_incr_rate: i32,
    pub special_skin: bool,
    pub current_mood: i32,
    pub is_locked: bool,
    pub finish_train: bool,
    pub is_high_quality: bool,
    pub train_hero_id: i32,
    pub total_finish_count: i32,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone)]
pub struct SkillInfo {
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RestInfo {
    pub building_uid: i64,
    pub rest_slot_id: i32,
}

#[derive(Debug, Clone)]
pub struct TagAttributeRate {
    pub attribute_id: i32,
    pub rate: i32,
}

#[derive(Debug, Clone)]
pub struct CritterInfo {
    pub uid: i64,
    pub define_id: i32,
    pub create_time: i64,
    pub efficiency: i32,
    pub patience: i32,
    pub lucky: i32,
    pub efficiency_incr_rate: i32,
    pub patience_incr_rate: i32,
    pub lucky_incr_rate: i32,
    pub special_skin: bool,
    pub current_mood: i32,
    pub lock: bool,
    pub finish_train: bool,
    pub is_high_quality: bool,
    pub train_hero_id: i32,
    pub total_finish_count: i32,
    pub name: String,
    pub skill_info: SkillInfo,
    pub rest_info: Option<RestInfo>,
    pub tag_attribute_rates: Vec<TagAttributeRate>,
    pub train_info: Option<()>, // Placeholder
    pub work_info: Option<()>,  // Placeholder
}

// Convert to proto
impl From<SkillInfo> for sonettobuf::CritterSkillInfo {
    fn from(info: SkillInfo) -> Self {
        sonettobuf::CritterSkillInfo { tags: info.tags }
    }
}

impl From<RestInfo> for sonettobuf::CritterRestInfo {
    fn from(info: RestInfo) -> Self {
        sonettobuf::CritterRestInfo {
            building_uid: Some(info.building_uid),
            rest_slot_id: Some(info.rest_slot_id),
        }
    }
}

impl From<TagAttributeRate> for sonettobuf::CritterTagAttributeRate {
    fn from(attr: TagAttributeRate) -> Self {
        sonettobuf::CritterTagAttributeRate {
            attribute_id: Some(attr.attribute_id),
            rate: Some(attr.rate),
        }
    }
}

impl From<CritterInfo> for sonettobuf::CritterInfo {
    fn from(c: CritterInfo) -> Self {
        sonettobuf::CritterInfo {
            uid: Some(c.uid),
            define_id: Some(c.define_id),
            create_time: Some((c.create_time / 1000) as i32),
            efficiency: Some(c.efficiency),
            patience: Some(c.patience),
            lucky: Some(c.lucky),
            efficiency_incr_rate: Some(c.efficiency_incr_rate),
            patience_incr_rate: Some(c.patience_incr_rate),
            lucky_incr_rate: Some(c.lucky_incr_rate),
            special_skin: Some(c.special_skin),
            current_mood: Some(c.current_mood),
            lock: Some(c.lock),
            finish_train: Some(c.finish_train),
            train_info: None,
            skill_info: Some(c.skill_info.into()),
            work_info: None,
            is_high_quality: Some(c.is_high_quality),
            rest_info: c.rest_info.map(Into::into),
            tag_attribute_rates: c.tag_attribute_rates.into_iter().map(Into::into).collect(),
            train_hero_id: Some(c.train_hero_id),
            total_finish_count: Some(c.total_finish_count),
            name: Some(c.name),
        }
    }
}
