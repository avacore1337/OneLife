use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum BlessingTypes {
    HeruclesStrength,
    AthenasWisdom,
}

pub const BLESSING_SIZE: usize = variant_count::<BlessingTypes>();
