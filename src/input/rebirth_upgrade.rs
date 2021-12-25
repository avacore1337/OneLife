use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum RebirthUpgradeTypes {
    AcceptingDeath,
    Skills,
    StartingItems1,
    StartingItems2,
    StartingFunds1,
    StartingFunds2,
    Privilege1,
    Privilege2,
    Privilege3,
    StatMemory1,
    EndItEarly,
    // GemSense, // gives you a huge multiplier to mining
    AutoWork,
    AutoLive,
    AutoBuyItem,
    AutoBuyTomb,
    Replay,
    TheDivine,
}

pub const REBIRTH_UPGRADE_SIZE: usize = variant_count::<RebirthUpgradeTypes>();

// impl Recordable for RebirthUpgradeTypes {
//     fn to_record_key(&self) -> String {
//         format!("Buy Rebirth Upgrade {:#?}", self)
//     }
// }
