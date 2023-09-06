extern crate variant_count;

use serde::{Deserialize, Serialize};
use strum::EnumIter;
use variant_count::VariantCount;

use super::Recordable;

#[derive(
    Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd, VariantCount,
)]
pub enum BlessingTypes {
    HeruclesStrength,
    AthenasWisdom,
    AfroditesCharm,
    PoseidonsSturdiness,
}

pub const BLESSING_SIZE: usize = BlessingTypes::VARIANT_COUNT;

impl Recordable for BlessingTypes {
    fn to_record_key(&self) -> String {
        format!("Buy Blessing {:#?}", self)
    }
}
