use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct RebirthStats {
    pub rebirth_count: u32,
    pub class_tier: u32,
    pub coins: f64,
    pub karma: f64,
    pub time_factor: f64,
}

impl RebirthStats {
    pub fn new() -> RebirthStats {
        RebirthStats {
            rebirth_count: 0,
            class_tier: 0,
            coins: 0.0,
            karma: 0.0,
            time_factor: 1.0,
        }
    }
}
