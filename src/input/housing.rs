extern crate variant_count;

use super::Recordable;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use variant_count::VariantCount;

#[derive(
    Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd, VariantCount,
)]
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

pub const HOUSING_SIZE: usize = HousingTypes::VARIANT_COUNT;

impl Recordable for HousingTypes {
    fn to_record_key(&self) -> String {
        format!("Set Housing {:#?}", self)
    }
}
