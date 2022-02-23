use super::Recordable;
use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(readonly)]
#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum HousingTypes {
    StoneFloor,
    ComfortableSpot,
    FilthyBarracks,
    CrampedBarracks,
    SharedRoom,
    LargeCloset,
    PrivateRoom,
    TinyAppartment,
    SmallAppartment,
    Appartment,
}

pub const HOUSING_SIZE: usize = variant_count::<HousingTypes>();

impl Recordable for HousingTypes {
    fn to_record_key(&self) -> String {
        format!("Set Housing {:#?}", self)
    }
}
