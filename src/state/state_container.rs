use super::items::Items;
use super::life_stats::LifeStats;
use super::rebirth_stats::RebirthStats;
use super::stats::BaseStats;
use crate::world::world::World;

use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct StateContainer {
    pub base_stats: BaseStats,
    pub rebirth_stats: RebirthStats,
    pub life_stats: LifeStats,
    pub items: Items,
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
            .starting_stats
            .clone(),
        rebirth_stats,
        life_stats: LifeStats::new(),
        items: Items::new(),
    }
}
