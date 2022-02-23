use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

use super::Recordable;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(readonly)]
#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum TombTypes {
    ShallowGrave,
    BurialPit,
    Grave,
    FamilyGrave,
    Tomb,
    Crypt,
    Mausuleum,
    Catacomb,
}

pub const TOMB_SIZE: usize = variant_count::<TombTypes>();

impl Recordable for TombTypes {
    fn to_record_key(&self) -> String {
        format!("Buy Tomb {:#?}", self)
    }
}
