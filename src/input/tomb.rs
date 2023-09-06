extern crate variant_count;

use serde::{Deserialize, Serialize};
use strum::EnumIter;
use variant_count::VariantCount;

use super::Recordable;

#[derive(
    Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd, VariantCount,
)]
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

pub const TOMB_SIZE: usize = TombTypes::VARIANT_COUNT;

impl Recordable for TombTypes {
    fn to_record_key(&self) -> String {
        format!("Buy Tomb {:#?}", self)
    }
}
