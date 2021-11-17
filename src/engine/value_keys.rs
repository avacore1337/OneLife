use crate::input::stat::StatTypes;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
// use std::mem::variant_count;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyValues {
    Happiness,
    Money,
    Coins,
    Str,
    Int,
    Cha,
    Con,
    Dex,
    Faith,
}

impl From<StatTypes> for KeyValues {
    fn from(stat: StatTypes) -> Self {
        match stat {
            StatTypes::Str => KeyValues::Str,
            StatTypes::Cha => KeyValues::Cha,
            StatTypes::Con => KeyValues::Con,
            StatTypes::Int => KeyValues::Int,
            StatTypes::Dex => KeyValues::Dex,
            StatTypes::Faith => KeyValues::Faith,
        }
    }
}

// pub const HOUSING_SIZE: usize = variant_count::<KeyValues>();
