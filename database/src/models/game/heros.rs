use anyhow::Result;
use serde::{Deserialize, Serialize};
use sonettobuf;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Hero {
    pub uid: i64,
    pub user_id: i64,
    pub hero_id: i32,
    pub create_time: i64,
    pub level: i32,
    pub exp: i32,
    pub rank: i32,
    pub breakthrough: i32,
    pub skin: i32,
    pub faith: i32,
    pub active_skill_level: i32,
    pub ex_skill_level: i32,
    pub is_new: bool,
    pub talent: i32,
    pub default_equip_uid: i64,
    pub duplicate_count: i32,
    pub use_talent_template_id: i32,
    pub talent_style_unlock: i32,
    pub talent_style_red: i32,
    pub is_favor: bool,
    pub destiny_rank: i32,
    pub destiny_level: i32,
    pub destiny_stone: i32,
    pub red_dot: i32,
    pub extra_str: String,
    // Base attributes
    pub base_hp: i32,
    pub base_attack: i32,
    pub base_defense: i32,
    pub base_mdefense: i32,
    pub base_technic: i32,
    pub base_multi_hp_idx: i32,
    pub base_multi_hp_num: i32,
    // Ex attributes
    pub ex_cri: i32,
    pub ex_recri: i32,
    pub ex_cri_dmg: i32,
    pub ex_cri_def: i32,
    pub ex_add_dmg: i32,
    pub ex_drop_dmg: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct HeroSpAttribute {
    pub hero_uid: i64,
    pub revive: i32,
    pub heal: i32,
    pub absorb: i32,
    pub defense_ignore: i32,
    pub clutch: i32,
    pub final_add_dmg: i32,
    pub final_drop_dmg: i32,
    pub normal_skill_rate: i32,
    pub play_add_rate: i32,
    pub play_drop_rate: i32,
    pub dizzy_resistances: i32,
    pub sleep_resistances: i32,
    pub petrified_resistances: i32,
    pub frozen_resistances: i32,
    pub disarm_resistances: i32,
    pub forbid_resistances: i32,
    pub seal_resistances: i32,
    pub cant_get_exskill_resistances: i32,
    pub del_ex_point_resistances: i32,
    pub stress_up_resistances: i32,
    pub control_resilience: i32,
    pub del_ex_point_resilience: i32,
    pub stress_up_resilience: i32,
    pub charm_resistances: i32,
    pub rebound_dmg: i32,
    pub extra_dmg: i32,
    pub reuse_dmg: i32,
    pub big_skill_rate: i32,
    pub clutch_dmg: i32,
}

impl From<HeroSpAttribute> for sonettobuf::HeroSpAttribute {
    fn from(sp: HeroSpAttribute) -> Self {
        sonettobuf::HeroSpAttribute {
            revive: Some(sp.revive),
            heal: Some(sp.heal),
            absorb: Some(sp.absorb),
            defense_ignore: Some(sp.defense_ignore),
            clutch: Some(sp.clutch),
            final_add_dmg: Some(sp.final_add_dmg),
            final_drop_dmg: Some(sp.final_drop_dmg),
            normal_skill_rate: Some(sp.normal_skill_rate),
            play_add_rate: Some(sp.play_add_rate),
            play_drop_rate: Some(sp.play_drop_rate),
            dizzy_resistances: Some(sp.dizzy_resistances),
            sleep_resistances: Some(sp.sleep_resistances),
            petrified_resistances: Some(sp.petrified_resistances),
            frozen_resistances: Some(sp.frozen_resistances),
            disarm_resistances: Some(sp.disarm_resistances),
            forbid_resistances: Some(sp.forbid_resistances),
            seal_resistances: Some(sp.seal_resistances),
            cant_get_exskill_resistances: Some(sp.cant_get_exskill_resistances),
            del_ex_point_resistances: Some(sp.del_ex_point_resistances),
            stress_up_resistances: Some(sp.stress_up_resistances),
            control_resilience: Some(sp.control_resilience),
            del_ex_point_resilience: Some(sp.del_ex_point_resilience),
            stress_up_resilience: Some(sp.stress_up_resilience),
            charm_resistances: Some(sp.charm_resistances),
            rebound_dmg: Some(sp.rebound_dmg),
            extra_dmg: Some(sp.extra_dmg),
            reuse_dmg: Some(sp.reuse_dmg),
            big_skill_rate: Some(sp.big_skill_rate),
            clutch_dmg: Some(sp.clutch_dmg),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct HeroEquipAttribute {
    pub id: i64,
    pub hero_uid: i64,
    pub equip_id: i32,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub mdefense: i32,
    pub technic: i32,
    pub multi_hp_idx: i32,
    pub multi_hp_num: i32,
}

impl From<HeroEquipAttribute> for sonettobuf::HeroEquipAttribute {
    fn from(e: HeroEquipAttribute) -> Self {
        sonettobuf::HeroEquipAttribute {
            id: Some(e.equip_id),
            equip_attr: Some(sonettobuf::HeroAttribute {
                hp: Some(e.hp),
                attack: Some(e.attack),
                defense: Some(e.defense),
                mdefense: Some(e.mdefense),
                technic: Some(e.technic),
                multi_hp_idx: Some(e.multi_hp_idx),
                multi_hp_num: Some(e.multi_hp_num),
            }),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct HeroSkin {
    pub hero_uid: i64,
    pub skin: i32,
    pub expire_sec: i32,
}

impl From<HeroSkin> for sonettobuf::SkinInfo {
    fn from(s: HeroSkin) -> Self {
        sonettobuf::SkinInfo {
            skin: Some(s.skin),
            expire_sec: Some(s.expire_sec),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct HeroTalentCube {
    pub hero_uid: i64,
    pub cube_id: i32,
    pub direction: i32,
    pub pos_x: i32,
    pub pos_y: i32,
}

impl From<HeroTalentCube> for sonettobuf::TalentCubeInfo {
    fn from(c: HeroTalentCube) -> Self {
        sonettobuf::TalentCubeInfo {
            cube_id: Some(c.cube_id),
            direction: Some(c.direction),
            pos_x: Some(c.pos_x),
            pos_y: Some(c.pos_y),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct HeroTalentTemplate {
    pub id: i64,
    pub hero_uid: i64,
    pub template_id: i32,
    pub name: String,
    pub style: i32,
}

#[derive(Debug, Clone)]
pub struct HeroData {
    pub record: Hero,
    pub passive_skill_levels: Vec<i32>,
    pub voices: Vec<i32>,
    pub voices_heard: Vec<i32>,
    pub skin_list: Vec<HeroSkin>,
    pub sp_attr: Option<HeroSpAttribute>,
    pub equip_attrs: Vec<HeroEquipAttribute>,
    pub item_unlocks: Vec<i32>,
    pub talent_cubes: Vec<HeroTalentCube>,
    pub talent_templates: Vec<(HeroTalentTemplate, Vec<HeroTalentCube>)>,
    pub destiny_stone_unlocks: Vec<i32>,
}

impl From<HeroData> for sonettobuf::HeroInfo {
    fn from(h: HeroData) -> Self {
        sonettobuf::HeroInfo {
            uid: h.record.uid,
            user_id: h.record.user_id,
            hero_id: h.record.hero_id,
            create_time: Some(h.record.create_time),
            level: Some(h.record.level),
            exp: Some(h.record.exp),
            rank: Some(h.record.rank),
            breakthrough: Some(h.record.breakthrough),
            skin: Some(h.record.skin),
            faith: Some(h.record.faith),
            active_skill_level: Some(h.record.active_skill_level),
            passive_skill_level: h.passive_skill_levels,
            ex_skill_level: Some(h.record.ex_skill_level),
            voice: h.voices,
            voice_heard: h.voices_heard,
            skin_info_list: h.skin_list.into_iter().map(Into::into).collect(),
            base_attr: Some(sonettobuf::HeroAttribute {
                hp: Some(h.record.base_hp),
                attack: Some(h.record.base_attack),
                defense: Some(h.record.base_defense),
                mdefense: Some(h.record.base_mdefense),
                technic: Some(h.record.base_technic),
                multi_hp_idx: Some(h.record.base_multi_hp_idx),
                multi_hp_num: Some(h.record.base_multi_hp_num),
            }),
            ex_attr: Some(sonettobuf::HeroExAttribute {
                cri: Some(h.record.ex_cri),
                recri: Some(h.record.ex_recri),
                cri_dmg: Some(h.record.ex_cri_dmg),
                cri_def: Some(h.record.ex_cri_def),
                add_dmg: Some(h.record.ex_add_dmg),
                drop_dmg: Some(h.record.ex_drop_dmg),
            }),
            sp_attr: h.sp_attr.map(Into::into),
            equip_attr_list: h.equip_attrs.into_iter().map(Into::into).collect(),
            is_new: Some(h.record.is_new),
            item_unlock: h.item_unlocks,
            talent: Some(h.record.talent),
            talent_cube_infos: h.talent_cubes.into_iter().map(Into::into).collect(),
            default_equip_uid: Some(h.record.default_equip_uid),
            duplicate_count: Some(h.record.duplicate_count),
            talent_templates: h
                .talent_templates
                .into_iter()
                .map(|(template, cubes)| sonettobuf::TalentTemplateInfo {
                    id: Some(template.template_id),
                    talent_cube_infos: cubes.into_iter().map(Into::into).collect(),
                    name: Some(template.name),
                    style: Some(template.style),
                })
                .collect(),
            use_talent_template_id: Some(h.record.use_talent_template_id),
            talent_style_unlock: Some(h.record.talent_style_unlock),
            talent_style_red: Some(h.record.talent_style_red),
            is_favor: Some(h.record.is_favor),
            destiny_rank: Some(h.record.destiny_rank),
            destiny_level: Some(h.record.destiny_level),
            destiny_stone: Some(h.record.destiny_stone),
            destiny_stone_unlock: h.destiny_stone_unlocks,
            red_dot: Some(h.record.red_dot),
            extra_str: Some(h.record.extra_str),
        }
    }
}

impl HeroData {
    /// Update hero's equipped skin
    pub async fn update_skin(&mut self, pool: &SqlitePool, skin_id: i32) -> Result<()> {
        sqlx::query("UPDATE heroes SET skin = ? WHERE uid = ?")
            .bind(skin_id)
            .bind(self.record.uid)
            .execute(pool)
            .await?;

        self.record.skin = skin_id;
        Ok(())
    }

    /// Update hero's destiny stone
    pub async fn update_destiny_stone(&mut self, pool: &SqlitePool, stone_id: i32) -> Result<()> {
        sqlx::query("UPDATE heroes SET destiny_stone = ? WHERE uid = ?")
            .bind(stone_id)
            .bind(self.record.uid)
            .execute(pool)
            .await?;

        self.record.destiny_stone = stone_id;
        Ok(())
    }

    /// Update hero's equipped gear
    pub async fn update_equipped_gear(&mut self, pool: &SqlitePool, equip_uid: i64) -> Result<()> {
        sqlx::query("UPDATE heroes SET default_equip_uid = ? WHERE uid = ?")
            .bind(equip_uid)
            .bind(self.record.uid)
            .execute(pool)
            .await?;

        self.record.default_equip_uid = equip_uid;
        Ok(())
    }

    /// Update assassin creed hero's equipped gear
    pub async fn update_special_equipped_gear(
        &mut self,
        pool: &SqlitePool,
        extra_str: String,
    ) -> Result<()> {
        sqlx::query("UPDATE heroes SET extra_str = ? WHERE uid = ?")
            .bind(&extra_str)
            .bind(self.record.uid)
            .execute(pool)
            .await?;

        self.record.extra_str = extra_str;
        Ok(())
    }

    /// Mark hero as favorite/unfavorite
    pub async fn toggle_favor(&mut self, pool: &SqlitePool) -> Result<()> {
        let new_favor = !self.record.is_favor;

        sqlx::query("UPDATE heroes SET is_favor = ? WHERE uid = ?")
            .bind(new_favor)
            .bind(self.record.uid)
            .execute(pool)
            .await?;

        self.record.is_favor = new_favor;
        Ok(())
    }

    /// Level up hero
    pub async fn level_up(
        &mut self,
        pool: &SqlitePool,
        new_level: i32,
        new_exp: i32,
    ) -> Result<()> {
        sqlx::query("UPDATE heroes SET level = ?, exp = ? WHERE uid = ?")
            .bind(new_level)
            .bind(new_exp)
            .bind(self.record.uid)
            .execute(pool)
            .await?;

        self.record.level = new_level;
        self.record.exp = new_exp;
        Ok(())
    }

    /// Rank up hero
    pub async fn rank_up(&mut self, pool: &SqlitePool, new_rank: i32) -> Result<()> {
        sqlx::query("UPDATE heroes SET rank = ? WHERE uid = ?")
            .bind(new_rank)
            .bind(self.record.uid)
            .execute(pool)
            .await?;

        self.record.rank = new_rank;
        Ok(())
    }

    /// Clear "new" flag
    pub async fn mark_as_seen(&mut self, pool: &SqlitePool) -> Result<()> {
        sqlx::query("UPDATE heroes SET is_new = 0 WHERE uid = ?")
            .bind(self.record.uid)
            .execute(pool)
            .await?;

        self.record.is_new = false;
        Ok(())
    }
}
