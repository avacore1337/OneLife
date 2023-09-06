extern crate variant_count;

use serde::{Deserialize, Serialize};
use strum::EnumIter;
use variant_count::VariantCount;

use super::Recordable;

#[derive(
    Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd, VariantCount,
)]
pub enum BoostItemTypes {
    Book,
    Shoe1,
    Clothes1,
    IronPickAxe,
    MiningGear,
    ExpertMiningGear,
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
    Dumbell5,
    Meditation4,
    Burial1,
    Burial2,
    Burial3,
    Burial4,
    BribeOfficial1,
    BribeOfficial2,
    BribeOfficial3,
    Tactics1,
    Tactics2,
    Tactics3,
    Tactics4,
    Tactics5,
    Book6,
    Shoe6,
    Clothes6,
}

pub const BOOST_ITEM_SIZE: usize = BoostItemTypes::VARIANT_COUNT;

impl Recordable for BoostItemTypes {
    fn to_record_key(&self) -> String {
        format!("Buy Item {:#?}", self)
    }
}
