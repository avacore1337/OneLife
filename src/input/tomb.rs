use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug)]
pub enum TombTypes {
    // Nothing,
    ShallowGrave,
    Tomb,
    Mausuleum,
}

pub const TOMB_SIZE: usize = variant_count::<TombTypes>();
