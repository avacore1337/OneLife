use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct BaseStats {
    pub str: f64,
    pub int: f64,
    pub cha: f64,
    pub con: f64,
    pub dex: f64,
    pub faith: f64,
}
