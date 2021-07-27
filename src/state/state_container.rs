use super::rebirth_stats::RebirthStats;
use super::stats::BaseStats;
use crate::world::world::World;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct StateContainer {
    pub base_stats: BaseStats,
    pub rebirth_stats: RebirthStats,
}

pub fn new_game(world: World) -> StateContainer {
    let rebirth_stats = RebirthStats { class_tier: 0 };
    rebirth(world, rebirth_stats)
}

pub fn rebirth(world: World, rebirth_stats: RebirthStats) -> StateContainer {
    StateContainer {
        base_stats: world
            .tiers
            .get(rebirth_stats.class_tier as usize)
            .expect("tier not implemented")
            .starting_stats
            .clone(),
        rebirth_stats,
    }
}
