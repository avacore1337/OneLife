extern crate variant_count;

use serde::{Deserialize, Serialize};
use strum::EnumIter;
use variant_count::VariantCount;

use super::Recordable;

#[derive(
    Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd, VariantCount,
)]
pub enum ActivityTypes {
    //Stat boosts
    Run,
    Studying,
    Training,
    Flirt,
    Praying,
    Acrobatics,
    //Skills
    Meditate,
    WarGames,
}

pub const ACTIVITY_SIZE: usize = ActivityTypes::VARIANT_COUNT;

impl Recordable for ActivityTypes {
    fn to_record_key(&self) -> String {
        format!("Set Activity {:#?}", self)
    }
}
