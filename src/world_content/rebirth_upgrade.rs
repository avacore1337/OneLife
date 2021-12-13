use crate::engine::intermediate_state::{Gain, IntermediateState};
use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::input::boost_item::BoostItemTypes;
use crate::input::rebirth_upgrade::{RebirthUpgradeTypes, REBIRTH_UPGRADE_SIZE};
use crate::state::rebirth_stats::Unlocks;
use crate::state::state_container::StateContainer;
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub struct RebirthUpgrade {
    pub name: RebirthUpgradeTypes,
    pub purchasing_cost: f64,
    pub description: &'static str,
    pub display_name: &'static str,
    pub required_tier: u32,
}

pub fn apply_starting_upgrade(state: &mut StateContainer, rebirth_upgrade: RebirthUpgradeTypes) {
    match rebirth_upgrade {
        RebirthUpgradeTypes::StartingItems1 => {
            state.items.boost_items[BoostItemTypes::Book as usize].is_purchased = true;
            state.items.boost_items[BoostItemTypes::Shoe1 as usize].is_purchased = true;
            state.items.boost_items[BoostItemTypes::RaggedClothes as usize].is_purchased = true;
        }
        _ => (),
    }
}

pub fn unlock(rebirth_upgrade: RebirthUpgradeTypes, unlocks: &mut Unlocks) {
    match rebirth_upgrade {
        RebirthUpgradeTypes::EndItEarly => {
            unlocks.can_end_early = true;
        }
        RebirthUpgradeTypes::AutoWork => {
            unlocks.can_auto_work = true;
        }
        RebirthUpgradeTypes::AutoLive => {
            unlocks.can_auto_living = true;
        }
        RebirthUpgradeTypes::AutoBuyItem => {
            unlocks.can_auto_buy_item = true;
        }
        RebirthUpgradeTypes::AutoBuyTomb => {
            unlocks.can_auto_buy_tomb = true;
        }
        RebirthUpgradeTypes::Replay => {
            unlocks.can_replay = true;
        }
        RebirthUpgradeTypes::TheDivine => {
            unlocks.has_faith = true;
        }
        _ => (),
    }
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
            display_name: "Accepting Death",
            required_tier: 1,
        },
        RebirthUpgradeTypes::StatMemory1 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 60.0,
            description: "Somehow your body remembers",
            display_name: "A feeling of the past",
            required_tier: 2,
        },
        RebirthUpgradeTypes::EndItEarly => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 600.0,
            description: "You can now commit a grave sin",
            display_name: "Ending It Early",
            required_tier: 3,
        },
        RebirthUpgradeTypes::StartingItems1 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 900.0,
            description: "Anything is better than nothing",
            display_name: "Starting Items",
            required_tier: 3,
        },
        RebirthUpgradeTypes::AutoWork => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 12.0,
            description: "You start making decisions out of habit",
            display_name: "Automate Work Progression",
            required_tier: 1,
        },
        RebirthUpgradeTypes::AutoLive => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 12.0,
            description: "If you can affort it, why not?",
            display_name: "Automate Housing Progression",
            required_tier: 1,
        },
        RebirthUpgradeTypes::AutoBuyItem => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 60.0,
            description: "Consumerism!",
            display_name: "Automate Buying of Items",
            required_tier: 2,
        },
        RebirthUpgradeTypes::AutoBuyTomb => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 160.0,
            description: "Put some more thought into it!",
            display_name: "Automate Buying of Tombs",
            required_tier: 3,
        },
        RebirthUpgradeTypes::Replay => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 6000.0,
            description: "Are you still playing?",
            display_name: "Replay",
            required_tier: 5,
        },
        RebirthUpgradeTypes::TheDivine => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 600_000.0,
            description: "Have a little faith",
            display_name: "Connect With The Gods",
            required_tier: 6,
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

pub fn should_be_visible_rebirth_upgrade(
    input_rebirth_upgrade: RebirthUpgradeTypes,
    game: &Game,
) -> bool {
    let rebirth_upgrade = &game.world.rebirth_upgrades[input_rebirth_upgrade as usize];
    rebirth_upgrade.required_tier <= game.state.rebirth_stats.class_tier
}

pub fn get_rebirth_upgrades() -> [RebirthUpgrade; REBIRTH_UPGRADE_SIZE] {
    let mut rebirth_upgrades: [MaybeUninit<RebirthUpgrade>; REBIRTH_UPGRADE_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in RebirthUpgradeTypes::iter() {
        rebirth_upgrades[name as usize].write(translate_rebirth_upgrade(name));
    }
    unsafe { mem::transmute(rebirth_upgrades) }
}
