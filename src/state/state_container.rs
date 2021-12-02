use super::activity::{get_activities, Activity};
use super::housing::{get_housings, Housing};
use super::items::Items;
use super::life_stats::LifeStats;
use super::rebirth_stats::RebirthStats;
use super::stats::Stat;
use super::tomb::{get_tombs, Tomb};
use super::work::{get_works, Work};
use crate::input::activity::ACTIVITY_SIZE;
use crate::input::housing::HOUSING_SIZE;
use crate::input::stat::STAT_SIZE;
use crate::input::tomb::TOMB_SIZE;
use crate::input::work::WORK_SIZE;
use crate::world_content::world::World;

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
}

pub fn new_game(world: &World) -> StateContainer {
    let rebirth_stats = RebirthStats::new();
    rebirth(world, rebirth_stats)
}

pub fn rebirth(world: &World, rebirth_stats: RebirthStats) -> StateContainer {
    let life_stats = LifeStats::new(world, &rebirth_stats);
    StateContainer {
        stats: world
            .tiers
            .get(rebirth_stats.class_tier as usize)
            .expect("tier not implemented")
            .starting_stats,
        rebirth_stats,
        life_stats,
        items: Items::new(),
        works: get_works(),
        activities: get_activities(),
        housing: get_housings(),
        tombs: get_tombs(),
    }
}
