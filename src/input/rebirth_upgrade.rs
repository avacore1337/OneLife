use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum RebirthUpgradeTypes {
    AcceptingDeath,
    AcceptingDeath2,
    AcceptingDeath3,
    StartingWealth1,
    StartingWealth2,
    StartingWealth3,
    LaborXp1,
    LaborXp2,
    SoldierXp1,
    SoldierXp2,
    Privilege1,
    Privilege2,
    Privilege3,
    // StatMemory1,
    EndItEarly,
    // GemSense, // gives you a huge multiplier to mining
    Skills,
    UnlockTactics,
    UnlockFaith,
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
