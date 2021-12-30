use super::rebirth_stats::RebirthStats;
use crate::WORLD;
use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LifeStats {
    pub age: f64,      // days
    pub lifespan: f64, // days
    pub health: f64,
    pub health_rate: f64,
    pub happiness: f64,
    pub is_dying: bool,
    pub dead: bool,
    pub current_tick: u32,
    pub replaying: bool,
}

impl LifeStats {
    pub fn new(rebirth_stats: &RebirthStats) -> LifeStats {
        let health = WORLD.tiers[rebirth_stats.tier as usize].starting_health;
        LifeStats {
            age: 15.0 * 365.0,
            lifespan: crate::BASE_LIFESPAN * (1.0 + health),
            health,
            health_rate: 0.0,
            happiness: 1.0,
            is_dying: false,
            dead: false,
            current_tick: 0,
            replaying: false,
        }
    }
}

// impl Default for LifeStats {
//     fn default() -> Self {
//         Self::new()
//     }
// }
