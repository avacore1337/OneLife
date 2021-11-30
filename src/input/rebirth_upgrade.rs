use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug)]
pub enum RebirthUpgradeTypes {
    AcceptingDeath,
    StatMemory1,
}

pub const REBIRTH_UPGRADE_SIZE: usize = variant_count::<RebirthUpgradeTypes>();
