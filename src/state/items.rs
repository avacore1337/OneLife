use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Items {
    pub money: f64,
}

impl Items {
    pub fn new() -> Items {
        Items { money: 0.0 }
    }
}

impl Default for Items {
    fn default() -> Self {
        Self::new()
    }
}
