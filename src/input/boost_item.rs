use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

use super::Recordable;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum BoostItemTypes {
    Book,
    Shoe1,
    RaggedClothes,
    IronPickAxe,
    Book2,
    Shoe2,
    FarmersClothes,
    MeditationMat,
    Leach,
    Flower1,
    Book3,
    Shoe3,
    FishingGear,
    CityClothes,
    // PitchFork,
    HealthKit,
    Dumbell,
    Dumbell2,
    Dumbell3,
    Flower2,
    Flower3,
    Incense,
}

pub const BOOST_ITEM_SIZE: usize = variant_count::<BoostItemTypes>();

impl Recordable for BoostItemTypes {
    fn to_record_key(&self) -> String {
        format!("Buy Item {:#?}", self)
    }
}
