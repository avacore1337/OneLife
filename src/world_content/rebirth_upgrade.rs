use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::icon::{Icon, IconType};
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
    pub effect_description: &'static str,
    pub icon: Icon,
}

pub fn apply_starting_upgrade(state: &mut StateContainer, rebirth_upgrade: RebirthUpgradeTypes) {
    match rebirth_upgrade {
        RebirthUpgradeTypes::StartingWealth1 => {
            state.items.money += 4000.0;
        }
        RebirthUpgradeTypes::StartingWealth2 => {
            state.boost_items[BoostItemTypes::Book as usize].is_purchased = true;
            state.boost_items[BoostItemTypes::Shoe1 as usize].is_purchased = true;
            state.boost_items[BoostItemTypes::Clothes1 as usize].is_purchased = true;
            state.items.money += 30000.0;
        }
        RebirthUpgradeTypes::StartingWealth3 => {
            state.items.money += 150_000.0;
            state.boost_items[BoostItemTypes::Book2 as usize].is_purchased = true;
            state.boost_items[BoostItemTypes::Shoe2 as usize].is_purchased = true;
            state.boost_items[BoostItemTypes::Dumbell as usize].is_purchased = true;
            state.boost_items[BoostItemTypes::Clothes2 as usize].is_purchased = true;
            state.boost_items[BoostItemTypes::Burial1 as usize].is_purchased = true;
        }
        RebirthUpgradeTypes::StartingWealth4 => {
            state.boost_items[BoostItemTypes::Dumbell2 as usize].is_purchased = true;
            state.boost_items[BoostItemTypes::Book3 as usize].is_purchased = true;
            state.boost_items[BoostItemTypes::Shoe3 as usize].is_purchased = true;
            state.boost_items[BoostItemTypes::Clothes3 as usize].is_purchased = true;
            state.boost_items[BoostItemTypes::Flower1 as usize].is_purchased = true;
            state.items.money += 1_000_000.0;
        }
        _ => (),
    }
}

pub fn unlock(rebirth_upgrade: RebirthUpgradeTypes, unlocks: &mut Unlocks) {
    match rebirth_upgrade {
        RebirthUpgradeTypes::UnlockMeditation => {
            unlocks.has_meditation = true;
        }
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
        RebirthUpgradeTypes::AutoBuyBlessing => {
            unlocks.can_auto_buy_blessing = true;
        }
        RebirthUpgradeTypes::QueueItems => {
            unlocks.can_queue_item = true;
        }
        RebirthUpgradeTypes::AutoBuyTomb => {
            unlocks.can_auto_buy_tomb = true;
        }
        RebirthUpgradeTypes::AutoRebirth => {
            unlocks.can_auto_rebirth = true;
        }
        RebirthUpgradeTypes::AutoEndEarly => {
            unlocks.can_auto_end_early = true;
        }
        RebirthUpgradeTypes::Replay => {
            unlocks.can_replay = true;
        }
        // RebirthUpgradeTypes::TheDivine => {
        //     unlocks.has_faith = true;
        // }
        RebirthUpgradeTypes::UnlockTactics => {
            unlocks.has_skills = true;
            unlocks.has_military_tactics = true;
        }
        RebirthUpgradeTypes::UnlockFaith => {
            unlocks.has_faith = true;
        }
        _ => (),
    }
}

impl RebirthUpgrade {
    pub fn get_upgrade_gains(&self, game: &mut Game) {
        let upgrade_state = &mut game.state.rebirth_stats.rebirth_upgrades[self.name as usize];
        if !upgrade_state.is_purchased {
            return;
        }

        let inter = &mut game.intermediate_state;
        match self.name {
            RebirthUpgradeTypes::GemKnowledge => {
                inter.add_multiplier(KeyValues::Mines, 100.0, self.display_name);
            }
            RebirthUpgradeTypes::AcceptingDeath => {
                inter.add_multiplier(KeyValues::Happiness, 1.7, self.display_name);
            }
            RebirthUpgradeTypes::AcceptingDeath2 => {
                inter.add_multiplier(KeyValues::Happiness, 1.3, self.display_name);
            }
            RebirthUpgradeTypes::AcceptingDeath3 => {
                inter.add_multiplier(KeyValues::Happiness, 1.3, self.display_name);
            }
            RebirthUpgradeTypes::AcceptingDeath4 => {
                inter.add_multiplier(KeyValues::Happiness, 1.5, self.display_name);
            }
            RebirthUpgradeTypes::Privilege1 => {
                inter.add_multiplier(KeyValues::Money, 1.2, self.display_name);
            }
            RebirthUpgradeTypes::Privilege2 => {
                inter.add_multiplier(KeyValues::Money, 1.2, self.display_name);
            }
            RebirthUpgradeTypes::Privilege3 => {
                inter.add_multiplier(KeyValues::Money, 1.2, self.display_name);
            }
            RebirthUpgradeTypes::LaborXp1 => {
                inter.add_multiplier(KeyValues::LaborXp, 1.5, self.display_name);
            }
            RebirthUpgradeTypes::LaborXp2 => {
                inter.add_multiplier(KeyValues::LaborXp, 1.5, self.display_name);
            }
            RebirthUpgradeTypes::LaborXp3 => {
                inter.add_multiplier(KeyValues::LaborXp, 1.5, self.display_name);
            }
            RebirthUpgradeTypes::SoldierXp1 => {
                inter.add_multiplier(KeyValues::SoldierXp, 2.0, self.display_name);
            }
            RebirthUpgradeTypes::SoldierXp2 => {
                inter.add_multiplier(KeyValues::SoldierXp, 2.0, self.display_name);
            }
            RebirthUpgradeTypes::SoldierXp3 => {
                inter.add_multiplier(KeyValues::SoldierXp, 2.0, self.display_name);
            }
            RebirthUpgradeTypes::BribeCharon1 => {
                inter.add_multiplier(KeyValues::Coins, 1.2, self.display_name);
            }
            RebirthUpgradeTypes::BribeCharon2 => {
                inter.add_multiplier(KeyValues::Coins, 1.2, self.display_name);
            }
            RebirthUpgradeTypes::BribeCharon3 => {
                inter.add_multiplier(KeyValues::Coins, 1.2, self.display_name);
            }
            _ => (),
        }
    }
}

pub fn translate_rebirth_upgrade(rebirth_upgrade: RebirthUpgradeTypes) -> RebirthUpgrade {
    match rebirth_upgrade {
        // -------------------------------------------------------
        // TIER 1
        RebirthUpgradeTypes::AcceptingDeath => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 3.0,
            description: "You feel happier now that you know that death isn't the end",
            display_name: "Accepting Death",
            effect_description: "Happiness 1.7x",
            required_tier: 1,
            icon: IconType::Happiness.into(),
        },
        RebirthUpgradeTypes::StartingWealth1 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 8.0,
            description: "Pocket change for some, a fortune for others",
            display_name: "Starting Money",
            effect_description: "Start with 4000 money",
            required_tier: 1,
            icon: IconType::Money.into(),
        },
        RebirthUpgradeTypes::Privilege1 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 12.0,
            description: "You have it easier than some at least",
            display_name: "Tiny Privilege",
            effect_description: "Money 1.2x",
            required_tier: 1,
            icon: IconType::Happiness.into(),
        },
        RebirthUpgradeTypes::LaborXp1 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 20.0,
            description: "Work harder",
            display_name: "Labor Experience",
            effect_description: "Labor job XP 1.5x",
            required_tier: 1,
            icon: IconType::Labor.into(),
        },
        RebirthUpgradeTypes::AutoWork => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 12.0,
            description: "You start making decisions out of habit",
            display_name: "Automate Work",
            effect_description: "Unlock Work progression",
            required_tier: 1,
            icon: IconType::Labor.into(),
        },

        // -------------------------------------------------------
        // TIER 2
        RebirthUpgradeTypes::BribeCharon1 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 30.0,
            description: "Sometimes they ferry payment is more expensive than it should be",
            display_name: "Bribe Charon 1",
            effect_description: "Coins 1.2x",
            required_tier: 2,
            icon: IconType::Coin.into(),
        },
        RebirthUpgradeTypes::StartingWealth2 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 80.0,
            description: "Pocket change for some, a fortune for others",
            display_name: "Starting Wealth 2",
            effect_description: "30k + items",
            required_tier: 1,
            icon: IconType::Money.into(),
        },
        RebirthUpgradeTypes::Privilege2 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 90.0,
            description: "Not the worst in town at least",
            display_name: "Minor Privilege",
            effect_description: "Money 1.2x",
            required_tier: 2,
            icon: IconType::Coin.into(),
        },
        RebirthUpgradeTypes::LaborXp2 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 80.0,
            description: "Work faster",
            display_name: "Labor Experience 2",
            effect_description: "Labor job XP 1.5x",
            required_tier: 2,
            icon: IconType::Labor.into(),
        },
        RebirthUpgradeTypes::AcceptingDeath2 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 200.0,
            description:
                "You now have a goal, to improve your life and have the best life possible",
            display_name: "Finding A Goal",
            effect_description: "Happiness 1.3x",
            required_tier: 2,
            icon: IconType::Happiness.into(),
        },
        RebirthUpgradeTypes::EndItEarly => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 300.0,
            description: "You can now commit a grave sin",
            display_name: "Ending It Early",
            effect_description: "You can now rebirth earlier",
            required_tier: 2,
            icon: IconType::Death.into(),
        },
        RebirthUpgradeTypes::AutoLive => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 12.0,
            description: "If you can affort it, why not?",
            display_name: "Automate Housing",
            effect_description: "Unlock automatic housing progression ",
            required_tier: 2,
            icon: IconType::Automate.into(),
        },
        RebirthUpgradeTypes::AutoBuyItem => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 60.0,
            description: "Consumerism!",
            display_name: "Automate Items",
            effect_description: "Unlock autobuying items",
            required_tier: 2,
            icon: IconType::Automate.into(),
        },
        RebirthUpgradeTypes::SoldierXp1 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 30.0,
            description: "Posture is important",
            display_name: "Soldier Experience",
            effect_description: "Soldier job XP 2x",
            required_tier: 2,
            icon: IconType::Soldier.into(),
        },

        // TIER 3 - 500
        RebirthUpgradeTypes::QueueItems => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 150.0,
            description: "tod!",
            display_name: "Unlock Item Queue",
            effect_description: "Shift click to queue items",
            required_tier: 3,
            icon: IconType::Automate.into(),
        },
        RebirthUpgradeTypes::AutoBuyTomb => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 250.0,
            description: "Put some more thought into it!",
            display_name: "Automate Tombs",
            effect_description: "Unlock autobuying tombs",
            required_tier: 3,
            icon: IconType::Tomb.into(),
        },
        RebirthUpgradeTypes::BribeCharon2 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 300.0,
            description: "Sometimes they ferry payment is more expensive than it should be",
            display_name: "Bribe Charon 2",
            effect_description: "Coins 1.2x",
            required_tier: 3,
            icon: IconType::Coin.into(),
        },
        RebirthUpgradeTypes::GemKnowledge => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 500.0,
            description:
                "You are told where all the miners found their big treasures during their lives",
            display_name: "Gem Knowledge",
            effect_description: "Mines income 100x",
            required_tier: 3,
            icon: IconType::Labor.into(),
        },
        RebirthUpgradeTypes::SoldierXp2 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 500.0,
            description: "Thrust! Slash! Lounge!",
            display_name: "Soldier Experience 2",
            effect_description: "Soldier job XP 2x",
            required_tier: 3,
            icon: IconType::Soldier.into(),
        },
        RebirthUpgradeTypes::StartingWealth3 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 800.0,
            description: "Anything is better than nothing",
            display_name: "Starting Wealth 3",
            effect_description: "150k + items",
            required_tier: 3,
            icon: IconType::Money.into(),
        },
        RebirthUpgradeTypes::UnlockTactics => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 1000.0,
            description: "A military genious in the making",
            display_name: "Unlock Tactics",
            effect_description: "Unlocks the skill tactics",
            required_tier: 3,
            icon: IconType::Tactics.into(),
        },
        RebirthUpgradeTypes::Privilege3 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 1500.0,
            description: "You have some perks based on your origins",
            display_name: "Lesser Privilege",
            effect_description: "Money 1.2x",
            required_tier: 3,
            icon: IconType::Money.into(),
        },
        RebirthUpgradeTypes::AcceptingDeath3 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 2000.0,
            description: "You know who you are. And that someone is going to be important one day",
            display_name: "Finding yourself",
            effect_description: "Happiness 1.3x",
            required_tier: 3,
            icon: IconType::Happiness.into(),
        },

        // TIER 4 - 5000
        RebirthUpgradeTypes::AutoRebirth => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 3_000.0,
            description: "Why would you want to not rebirth?",
            display_name: "Automate Rebirth",
            effect_description: "Unlock automatic rebirth",
            required_tier: 4,
            icon: IconType::Death.into(),
        },
        RebirthUpgradeTypes::BribeCharon3 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 3_000.0,
            description: "Sometimes they ferry payment is more expensive than it should be",
            display_name: "Bribe Charon 3",
            effect_description: "Coins 1.2x",
            required_tier: 4,
            icon: IconType::Coin.into(),
        },
        RebirthUpgradeTypes::UnlockMeditation => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 4_000.0,
            description: "You can be good at things now",
            display_name: "Meditation",
            effect_description: "Unlocks the Meditation skill",
            required_tier: 4,
            icon: IconType::Mindfull.into(),
        },
        RebirthUpgradeTypes::SoldierXp3 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 5000.0,
            description: "I'll make a *man*, out of youuuuu!",
            display_name: "Soldier Experience 3",
            effect_description: "Soldier job XP 2x",
            required_tier: 4,
            icon: IconType::Soldier.into(),
        },
        RebirthUpgradeTypes::LaborXp3 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 8000.0,
            description: "Work stronger",
            display_name: "Labor Experience 3",
            effect_description: "Labor job XP 1.5x",
            required_tier: 4,
            icon: IconType::Labor.into(),
        },
        RebirthUpgradeTypes::StartingWealth4 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 8000.0,
            description: "Anything is better than nothing",
            display_name: "Starting Wealth",
            effect_description: "1m + items",
            required_tier: 4,
            icon: IconType::Money.into(),
        },
        RebirthUpgradeTypes::UnlockFaith => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 10_000.0,
            description: "Believing in gods is not that hard when they actualy exists",
            display_name: "Unlock Faith",
            effect_description: "Unlocks the Faith stat",
            required_tier: 4,
            icon: IconType::Faith.into(),
        },
        RebirthUpgradeTypes::AcceptingDeath4 => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 20_000.0,
            description: "You know who you are. And that someone is going to be important one day",
            display_name: "Finding Happiness",
            effect_description: "Happiness 1.5x",
            required_tier: 4,
            icon: IconType::Happiness.into(),
        },
        RebirthUpgradeTypes::AutoEndEarly => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 30_000.0,
            description: "Might as well end it all",
            display_name: "AutoEndEarly",
            effect_description: "End early based on criteria",
            required_tier: 4,
            icon: IconType::Death.into(),
        },

        // TIER 5 - 50 000
        RebirthUpgradeTypes::AutoBuyBlessing => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 6000.0,
            description: "You should visit your local church regularly",
            display_name: "Automate Blessings",
            effect_description: "Unlock autobuying blessings",
            required_tier: 5,
            icon: IconType::Automate.into(),
        },
        RebirthUpgradeTypes::Replay => RebirthUpgrade {
            name: rebirth_upgrade,
            purchasing_cost: 6_000.0,
            description: "Are you still playing?",
            display_name: "Replay",
            effect_description: "You can replay your inputs",
            required_tier: 5,
            icon: IconType::Automate.into(),
        },
        // TIER 6 - 500 000
        // RebirthUpgradeTypes::TheDivine => RebirthUpgrade {
        //     name: rebirth_upgrade,
        //     purchasing_cost: 600_000.0,
        //     description: "Have a little faith",
        //     display_name: "Connect With The Gods",
        //     effect_description: "todo",
        //     required_tier: 6,
        //     icon: IconType::Happiness.into(),
        // },
        // RebirthUpgradeTypes::StatMemory1 => RebirthUpgrade {
        //     name: rebirth_upgrade,
        //     purchasing_cost: 60.0,
        //     description: "Somehow your body remembers",
        //     display_name: "A feeling of the past",
        //     effect_description: "todo",
        //     required_tier: 2,
        //     icon: IconType::Happiness.into(),
        // },
    }
}

pub fn should_unlock_rebirth_upgrade(
    input_rebirth_upgrade: RebirthUpgradeTypes,
    game: &Game,
) -> bool {
    let rebirth_upgrade = &game.world.rebirth_upgrades[input_rebirth_upgrade as usize];
    let too_low_tier = rebirth_upgrade.required_tier > game.state.rebirth_stats.tier;
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
    if rebirth_upgrade.required_tier > game.state.rebirth_stats.tier {
        return false;
    }
    match input_rebirth_upgrade {
        RebirthUpgradeTypes::StartingWealth2 => {
            game.state.rebirth_stats.rebirth_upgrades[RebirthUpgradeTypes::StartingWealth1 as usize]
                .is_purchased
        }
        RebirthUpgradeTypes::StartingWealth3 => {
            game.state.rebirth_stats.rebirth_upgrades[RebirthUpgradeTypes::StartingWealth2 as usize]
                .is_purchased
        }
        RebirthUpgradeTypes::StartingWealth4 => {
            game.state.rebirth_stats.rebirth_upgrades[RebirthUpgradeTypes::StartingWealth3 as usize]
                .is_purchased
        }
        _ => true,
    }
}

pub fn get_rebirth_upgrades() -> [RebirthUpgrade; REBIRTH_UPGRADE_SIZE] {
    let mut rebirth_upgrades: [MaybeUninit<RebirthUpgrade>; REBIRTH_UPGRADE_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in RebirthUpgradeTypes::iter() {
        rebirth_upgrades[name as usize].write(translate_rebirth_upgrade(name));
    }
    unsafe { mem::transmute(rebirth_upgrades) }
}
