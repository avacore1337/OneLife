use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum StatTypes {
    Con,
    Int,
    Str,
    Cha,
    Faith,
    Dex,
}

pub const STAT_SIZE: usize = variant_count::<StatTypes>();
