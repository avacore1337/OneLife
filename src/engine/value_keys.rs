use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

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

pub const HOUSING_SIZE: usize = variant_count::<KeyValues>();
