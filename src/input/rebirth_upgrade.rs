use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum RebirthUpgradeTypes {
    AcceptingDeath,
    StatMemory1,
    EndItEarly,
    // GemSense, // gives you a huge multiplier to mining
    AutoWork, // automatically progresses current job
    AutoLive, // automatically bumps place to live if income > cost * 1.5
    AutoBuyItem,
    AutoBuyTomb,
    Replay,
    TheDivine,
}

pub const REBIRTH_UPGRADE_SIZE: usize = variant_count::<RebirthUpgradeTypes>();
