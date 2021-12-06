use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
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

pub const TOMB_SIZE: usize = variant_count::<TombTypes>();
