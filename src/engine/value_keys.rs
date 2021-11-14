use crate::input::stat::Stat;
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

impl From<Stat> for KeyValues {
    fn from(stat: Stat) -> Self {
        match stat {
            Stat::Str => KeyValues::Str,
            Stat::Cha => KeyValues::Cha,
            Stat::Con => KeyValues::Con,
            Stat::Int => KeyValues::Int,
            Stat::Dex => KeyValues::Dex,
            Stat::Faith => KeyValues::Faith,
        }
    }
}

// pub const HOUSING_SIZE: usize = variant_count::<KeyValues>();
