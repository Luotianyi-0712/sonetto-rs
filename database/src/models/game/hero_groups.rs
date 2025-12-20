use sonettobuf;
use sqlx::FromRow;

#[allow(dead_code)]
#[derive(Debug, Clone, FromRow)]
pub struct HeroGroupCommon {
    pub id: i64,
    pub user_id: i64,
    pub group_id: i32,
    pub name: String,
    pub cloth_id: i32,
    pub assist_boss_id: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, FromRow)]
pub struct HeroGroupType {
    pub id: i64,
    pub user_id: i64,
    pub type_id: i32,
    pub current_select: i32,
    pub group_id: Option<i32>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone)]
pub struct HeroGroupEquip {
    pub index: i32,
    pub equip_uids: Vec<i64>,
}

impl From<HeroGroupEquip> for sonettobuf::HeroGroupEquip {
    fn from(equip: HeroGroupEquip) -> Self {
        sonettobuf::HeroGroupEquip {
            index: Some(equip.index),
            equip_uid: equip.equip_uids,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HeroGroupInfo {
    pub group_id: i32,
    pub hero_list: Vec<i64>,
    pub name: String,
    pub cloth_id: i32,
    pub equips: Vec<HeroGroupEquip>,
    pub activity104_equips: Vec<HeroGroupEquip>,
    pub assist_boss_id: i32,
}

impl From<HeroGroupInfo> for sonettobuf::HeroGroupInfo {
    fn from(info: HeroGroupInfo) -> Self {
        sonettobuf::HeroGroupInfo {
            group_id: info.group_id,
            hero_list: info.hero_list,
            name: Some(info.name),
            cloth_id: Some(info.cloth_id),
            equips: info.equips.into_iter().map(Into::into).collect(),
            activity104_equips: info
                .activity104_equips
                .into_iter()
                .map(Into::into)
                .collect(),
            assist_boss_id: Some(info.assist_boss_id),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HeroGroupTypeInfo {
    pub type_id: i32,
    pub current_select: i32,
    pub group_info: Option<HeroGroupInfo>,
}

impl From<HeroGroupTypeInfo> for sonettobuf::HeroGourpType {
    fn from(info: HeroGroupTypeInfo) -> Self {
        sonettobuf::HeroGourpType {
            id: Some(info.type_id),
            current_select: Some(info.current_select),
            group_info: info.group_info.map(Into::into),
        }
    }
}
