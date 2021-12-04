use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum BoostItemTypes {
    Book,
    Dumbell,
    RaggedClothes,
    IronPickAxe,
    Book2,
    Dumbell2,
    FarmersClothes,
    Book3,
    Dumbell3,
    FishingGear,
    CityClothes,
    Leach,
    PitchFork,
    HealthKit,
}

pub const BOOST_ITEM_SIZE: usize = variant_count::<BoostItemTypes>();
