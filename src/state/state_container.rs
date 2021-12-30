use super::activity::{get_activities, Activity};
use super::blessing::{get_blessings, Blessing};
use super::housing::{get_housings, Housing};
use super::items::Items;
use super::life_stats::LifeStats;
use super::rebirth_stats::RebirthStats;
use super::skill::{get_skills, Skill};
use super::stats::Stat;
use super::tomb::{get_tombs, Tomb};
use super::work::{get_works, Work};
use crate::input::activity::ACTIVITY_SIZE;
use crate::input::blessing::BLESSING_SIZE;
use crate::input::housing::HOUSING_SIZE;
use crate::input::rebirth_upgrade::RebirthUpgradeTypes;
use crate::input::skill::SKILL_SIZE;
use crate::input::stat::STAT_SIZE;
use crate::input::tomb::TOMB_SIZE;
use crate::input::work::{WorkTypes, WORK_SIZE};
use crate::world_content::rebirth_upgrade::apply_starting_upgrade;
use crate::WORLD;
use strum::IntoEnumIterator;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StateContainer {
    pub stats: [Stat; STAT_SIZE],
    pub rebirth_stats: RebirthStats,
    pub life_stats: LifeStats,
    pub items: Items,
    pub works: [Work; WORK_SIZE],
    pub activities: [Activity; ACTIVITY_SIZE],
    pub housing: [Housing; HOUSING_SIZE],
    pub tombs: [Tomb; TOMB_SIZE],
    pub blessings: [Blessing; BLESSING_SIZE],
    pub skills: [Skill; SKILL_SIZE],
}

impl Default for StateContainer {
    fn default() -> StateContainer {
        let rebirth_stats = RebirthStats::default();
        rebirth(rebirth_stats)
    }
}

pub fn rebirth(rebirth_stats: RebirthStats) -> StateContainer {
    let life_stats = LifeStats::new(&rebirth_stats);
    let mut state = StateContainer {
        stats: WORLD
            .tiers
            .get(rebirth_stats.tier as usize)
            .expect("tier not implemented")
            .starting_stats
            .clone(),
        rebirth_stats,
        life_stats,
        items: Items::new(),
        works: get_works(),
        activities: get_activities(),
        housing: get_housings(),
        tombs: get_tombs(),
        blessings: get_blessings(),
        skills: get_skills(),
    };
    for upgrade_type in RebirthUpgradeTypes::iter() {
        if state.rebirth_stats.rebirth_upgrades[upgrade_type as usize].is_purchased {
            apply_starting_upgrade(&mut state, upgrade_type);
        }
    }
    for work_type in WorkTypes::iter() {
        state.works[work_type as usize].max_job_levels =
            state.rebirth_stats.max_job_levels[work_type as usize];
    }
    state
}
