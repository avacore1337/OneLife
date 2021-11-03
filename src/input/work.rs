use serde::{Deserialize, Serialize};
use strum::EnumIter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug)]
pub enum Work {
    Mines,
    Fields,
    Servant,
    Teacher,
    Farm,
}
