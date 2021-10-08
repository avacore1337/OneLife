use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct LifeStats {
    pub age: f64,      // days
    pub lifespan: f64, // days
    pub health: f64,   // days
    pub happiness: f64,
    pub dead: bool,
}

impl LifeStats {
    pub fn new() -> LifeStats {
        LifeStats {
            age: 15.0 * 365.0,
            lifespan: 70.0 * 365.0,
            health: 0.0,
            happiness: 1.0,
            dead: false,
        }
    }
}
