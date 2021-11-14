use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug)]
pub enum Stat {
    Str,
    Int,
    Cha,
    Con,
    Dex,
    Faith,
}

pub const STAT_SIZE: usize = variant_count::<Stat>();
