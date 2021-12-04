use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum BoostItemTypes {
    Book,
    Shoe1,
    RaggedClothes,
    IronPickAxe,
    Book2,
    Shoe2,
    FarmersClothes,
    Book3,
    Shoe3,
    FishingGear,
    CityClothes,
    Leach,
    PitchFork,
    HealthKit,
    Dumbell,
    Dumbell2,
    Dumbell3,
}

pub const BOOST_ITEM_SIZE: usize = variant_count::<BoostItemTypes>();
