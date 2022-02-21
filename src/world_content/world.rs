#![allow(non_snake_case)]
use serbia::serbia;
use serde::Serialize;
use std::sync::Mutex;

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
use super::work::{get_works, Work};
use crate::icon::{get_icons, Icon};
use crate::input::activity::{ActivityTypes, ACTIVITY_SIZE};
use crate::input::blessing::BLESSING_SIZE;
use crate::input::boost_item::BOOST_ITEM_SIZE;
use crate::input::housing::{HousingTypes, HOUSING_SIZE};
use crate::input::rebirth_upgrade::{RebirthUpgradeTypes, REBIRTH_UPGRADE_SIZE};
use crate::input::skill::SKILL_SIZE;
use crate::input::stat::STAT_SIZE;
use crate::input::tomb::TOMB_SIZE;
use crate::input::work::{WorkTypes, WORK_SIZE};
use crate::input_mapping::InputMapping;
use std::collections::BTreeMap;

#[serbia]
#[derive(Serialize)]
pub struct World {
    activities: [Activity; ACTIVITY_SIZE],
    pub blessings: [Blessing; BLESSING_SIZE],
    // #[serde(serialize_with = "<[_]>::serialize")]
    pub boost_items: [BoostItem; BOOST_ITEM_SIZE],
    housing: [Housing; HOUSING_SIZE],
    pub rebirth_upgrades: [RebirthUpgrade; REBIRTH_UPGRADE_SIZE],
    pub settings: Settings,
    pub skills: [Skill; SKILL_SIZE],
    pub stats: [Stat; STAT_SIZE],
    pub tiers: Vec<Tier>,
    pub tombs: [Tomb; TOMB_SIZE],
    works: [Work; WORK_SIZE],
    #[serde(skip_serializing)]
    pub input_mapping: Mutex<InputMapping>,
    pub icons: BTreeMap<String, Icon>,
}

impl World {
    pub fn get_activity(&self, activity: ActivityTypes) -> &Activity {
        &self.activities[activity as usize]
    }
    pub fn get_housing(&self, housing: HousingTypes) -> &Housing {
        &self.housing[housing as usize]
    }
    pub fn get_rebirth_upgrade(&self, rebirth_upgrade: RebirthUpgradeTypes) -> &RebirthUpgrade {
        &self.rebirth_upgrades[rebirth_upgrade as usize]
    }
    pub fn get_work(&self, work: WorkTypes) -> &Work {
        &self.works[work as usize]
    }
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
            stats: get_stats(),
            settings: Settings::default(),
            blessings: get_blessings(),
            skills: get_skills(),
            input_mapping: Mutex::new(InputMapping::default()),
            icons: get_icons(),
        }
    }
}
