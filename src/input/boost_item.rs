use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

use super::Recordable;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum BoostItemTypes {
    Book,
    Shoe1,
    Clothes1,
    // IronPickAxe,
    Book2,
    Shoe2,
    Clothes2,
    Meditation1,
    Leach,
    Flower1,
    Book3,
    Shoe3,
    FishingGear,
    Clothes3,
    PitchFork,
    HealthKit,
    Book4,
    Shoe4,
    Dumbell,
    Dumbell2,
    IronAxe,
    Dumbell3,
    Flower2,
    Flower3,
    Flower4,
    Meditation2,
    Clothes4,
    Meditation3,
    Dumbell4,
    Book5,
    Shoe5,
    Clothes5,
    Meditation4,
    Burial1,
    Burial2,
    Burial3,
    Burial4,
    BribeOfficial1,
    BribeOfficial2,
    BribeOfficial3,
}

pub const BOOST_ITEM_SIZE: usize = variant_count::<BoostItemTypes>();

impl Recordable for BoostItemTypes {
    fn to_record_key(&self) -> String {
        format!("Buy Item {:#?}", self)
    }
}
