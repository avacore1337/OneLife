use super::activity::{get_activities, Activity};
use super::blessing::{get_blessings, Blessing};
use super::boost_item::{get_boost_items, BoostItem};
use super::housing::{get_housings, Housing};
use super::rebirth_upgrade::{get_rebirth_upgrades, RebirthUpgrade};
use super::settings::Settings;
use super::skill::{get_skills, Skill};
use super::stat::{get_stats, Stat};
use super::tier::{init_tiers, Tier};
use super::tomb::{get_tombs, Tomb};
use super::tutorial::get_tutorial_texts;
use super::work::{get_works, Work};
use crate::input::activity::ACTIVITY_SIZE;
use crate::input::blessing::BLESSING_SIZE;
use crate::input::boost_item::BOOST_ITEM_SIZE;
use crate::input::housing::HOUSING_SIZE;
use crate::input::rebirth_upgrade::REBIRTH_UPGRADE_SIZE;
use crate::input::skill::SKILL_SIZE;
use crate::input::stat::STAT_SIZE;
use crate::input::tomb::TOMB_SIZE;
use crate::input::work::WORK_SIZE;
use serde::Serialize;

#[derive(Serialize)]
pub struct World {
    pub tiers: Vec<Tier>,
    pub works: [Work; WORK_SIZE],
    pub activities: [Activity; ACTIVITY_SIZE],
    pub housing: [Housing; HOUSING_SIZE],
    pub boost_items: [BoostItem; BOOST_ITEM_SIZE],
    pub tombs: [Tomb; TOMB_SIZE],
    pub rebirth_upgrades: [RebirthUpgrade; REBIRTH_UPGRADE_SIZE],
    pub tutorial_texts: Vec<&'static str>,
    pub stats: [Stat; STAT_SIZE],
    pub settings: Settings,
    pub blessings: [Blessing; BLESSING_SIZE],
    pub skills: [Skill; SKILL_SIZE],
}

impl Default for World {
    fn default() -> World {
        World {
            tiers: init_tiers(),
            works: get_works(),
            housing: get_housings(),
            boost_items: get_boost_items(),
            activities: get_activities(),
            tombs: get_tombs(),
            rebirth_upgrades: get_rebirth_upgrades(),
            tutorial_texts: get_tutorial_texts(),
            stats: get_stats(),
            settings: Settings::default(),
            blessings: get_blessings(),
            skills: get_skills(),
        }
    }
}
