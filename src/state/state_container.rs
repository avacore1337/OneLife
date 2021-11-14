use super::items::Items;
use super::life_stats::LifeStats;
use super::rebirth_stats::RebirthStats;
use super::stats::Stat;
use super::work::{get_works, Work};
use crate::input::stat::STAT_SIZE;
use crate::input::work::WORK_SIZE;
use crate::world_content::world::World;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StateContainer {
    pub base_stats: [Stat; STAT_SIZE],
    pub rebirth_stats: RebirthStats,
    pub life_stats: LifeStats,
    pub items: Items,
    pub works: [Work; WORK_SIZE],
}

pub fn new_game(world: &World) -> StateContainer {
    let rebirth_stats = RebirthStats::new();
    rebirth(world, rebirth_stats)
}

pub fn rebirth(world: &World, rebirth_stats: RebirthStats) -> StateContainer {
    StateContainer {
        base_stats: world
            .tiers
            .get(rebirth_stats.class_tier as usize)
            .expect("tier not implemented")
            .starting_stats,
        rebirth_stats,
        life_stats: LifeStats::new(),
        items: Items::new(),
        works: get_works(),
    }
}
