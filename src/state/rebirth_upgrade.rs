use crate::input::rebirth_upgrade::{RebirthUpgradeTypes, REBIRTH_UPGRADE_SIZE};
use serde::{Deserialize, Serialize};
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RebirthUpgrade {
    pub name: RebirthUpgradeTypes,
    pub is_purchased: bool,
    pub is_unlocked: bool,
    pub is_visible: bool,
}

impl RebirthUpgrade {
    pub fn new(rebirth_upgrade: RebirthUpgradeTypes) -> RebirthUpgrade {
        RebirthUpgrade {
            name: rebirth_upgrade,
            is_purchased: false,
            is_unlocked: false,
            is_visible: true,
        }
    }
}

pub fn get_rebirth_upgrades() -> [RebirthUpgrade; REBIRTH_UPGRADE_SIZE] {
    let mut rebirth_upgrades: [MaybeUninit<RebirthUpgrade>; REBIRTH_UPGRADE_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in RebirthUpgradeTypes::iter() {
        rebirth_upgrades[name as usize].write(RebirthUpgrade::new(name));
    }
    unsafe { mem::transmute(rebirth_upgrades) }
}
