use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug)]
pub enum RebirthUpgradeTypes {
    AcceptingDeath,
    StatMemory1,
    // EndItEarly,
    // GemSense // gives you a huge multiplier to mining
}

pub const REBIRTH_UPGRADE_SIZE: usize = variant_count::<RebirthUpgradeTypes>();
