use super::rebirth_stats::RebirthStats;
use crate::world_content::world::World;
use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct LifeStats {
    pub age: f64,      // days
    pub lifespan: f64, // days
    pub health: f64,   // days
    pub happiness: f64,
    pub dead: bool,
}

impl LifeStats {
    pub fn new(world: &World, rebirth_stats: &RebirthStats) -> LifeStats {
        let health = world.tiers[rebirth_stats.class_tier as usize].starting_health;
        LifeStats {
            age: 15.0 * 365.0,
            lifespan: crate::BASE_LIFESPAN * (1.0 + health),
            health,
            happiness: 1.0,
            dead: false,
        }
    }
}

// impl Default for LifeStats {
//     fn default() -> Self {
//         Self::new()
//     }
// }
