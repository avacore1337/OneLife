use crate::engine::intermediate_state::{Gain, IntermediateState};
use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::input::rebirth_upgrade::{RebirthUpgradeTypes, REBIRTH_UPGRADE_SIZE};
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub struct RebirthUpgrade {
    pub name: RebirthUpgradeTypes,
    pub purchasing_cost: f64,
    pub description: &'static str,
    pub required_tier: u32,
}

impl Gain for RebirthUpgrade {
    fn gain(&self, intermediate: &mut IntermediateState) {
        match self.name {
            RebirthUpgradeTypes::AcceptingDeath => {
                intermediate.add_multiplier(KeyValues::Happiness, 2.0, "AcceptingDeath");
            }
            _ => (),
        }
    }
}
pub const fn translate_rebirth_upgrade(rebirth_upgrade: RebirthUpgradeTypes) -> RebirthUpgrade {
    match rebirth_upgrade {
        RebirthUpgradeTypes::AcceptingDeath => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 4.0,
            description: "You feel happier now that you know that death isn't the end",
            required_tier: 1,
        },
        RebirthUpgradeTypes::StatMemory1 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 60.0,
            description: "Somehow your body remembers",
            required_tier: 2,
        },
    }
}

pub fn should_unlock_rebirth_upgrade(
    input_rebirth_upgrade: RebirthUpgradeTypes,
    game: &Game,
) -> bool {
    let rebirth_upgrade = &game.world.rebirth_upgrades[input_rebirth_upgrade as usize];
    let too_low_tier = rebirth_upgrade.required_tier > game.state.rebirth_stats.class_tier;
    let cant_afford = rebirth_upgrade.purchasing_cost > game.state.rebirth_stats.coins;
    if too_low_tier || cant_afford {
        return false;
    }
    match input_rebirth_upgrade {
        RebirthUpgradeTypes::AcceptingDeath => true,
        _ => true,
    }
}

pub fn should_be_visable_rebirth_upgrade(
    input_rebirth_upgrade: RebirthUpgradeTypes,
    game: &Game,
) -> bool {
    let rebirth_upgrade = &game.world.rebirth_upgrades[input_rebirth_upgrade as usize];
    rebirth_upgrade.required_tier <= game.state.rebirth_stats.class_tier + 1
}

pub fn get_rebirth_upgrades() -> [RebirthUpgrade; REBIRTH_UPGRADE_SIZE] {
    let mut rebirth_upgrades: [MaybeUninit<RebirthUpgrade>; REBIRTH_UPGRADE_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in RebirthUpgradeTypes::iter() {
        rebirth_upgrades[name as usize].write(translate_rebirth_upgrade(name));
    }
    unsafe { mem::transmute(rebirth_upgrades) }
}
