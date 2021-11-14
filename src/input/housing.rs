use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy)]
pub enum Housing {
    StoneFloor,
    ComfortableSpot,
    FilthyBarracks,
    CrampedBarracks,
}

pub const HOUSING_SIZE: usize = variant_count::<Housing>();
