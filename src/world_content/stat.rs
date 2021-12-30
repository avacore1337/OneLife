use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::input::stat::{StatTypes, STAT_SIZE};
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize, Clone)]
pub struct Stat {
    pub name: StatTypes,
    pub description: &'static str,
    pub display_name: &'static str,
    pub required_tier: u32,
}

impl Stat {
    pub fn get_stats_gains(&self, game: &mut Game) {
        let stat_state = &mut game.state.stats[self.name as usize];
        if !stat_state.is_visible {
            return;
        }

        match self.name {
            StatTypes::Con => {
                game.intermediate_state.add_multiplier(
                    KeyValues::Health,
                    0.05 * stat_state.level,
                    self.display_name,
                );
            }
            StatTypes::Cha => {
                game.intermediate_state.add_multiplier(
                    KeyValues::Coins,
                    1.0 + 0.05 * stat_state.level,
                    self.display_name,
                );
            }
            _ => {}
        }
    }
}

pub const fn translate_stat(stat: StatTypes) -> Stat {
    match stat {
        StatTypes::Str => Stat {
            name: stat,
            description: "Do you even lift?",
            display_name: "Strength",
            required_tier: 2,
        },
        StatTypes::Cha => Stat {
            name: stat,
            description: "Being liked has it's benifits",
            display_name: "Charisma",
            required_tier: 1,
        },
        StatTypes::Dex => Stat {
            name: stat,
            description: "Bendy!",
            display_name: "Dexterity",
            required_tier: 7,
        },
        StatTypes::Int => Stat {
            name: stat,
            description: "Brainiac",
            display_name: "Intelligence",
            required_tier: 0,
        },
        StatTypes::Con => Stat {
            name: stat,
            description: "Endure the pain!",
            display_name: "Constitution",
            required_tier: 0,
        },
        StatTypes::Faith => Stat {
            name: stat,
            description: "Have some",
            display_name: "Faith",
            required_tier: 5,
        },
    }
}

pub fn should_be_visible_stat(input_stat: StatTypes, game: &Game) -> bool {
    let stat = &game.world.stats[input_stat as usize];
    game.state.rebirth_stats.tier >= stat.required_tier
}

pub fn get_stats() -> [Stat; STAT_SIZE] {
    let mut stats: [MaybeUninit<Stat>; STAT_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in StatTypes::iter() {
        stats[name as usize].write(translate_stat(name));
    }
    unsafe { mem::transmute(stats) }
}
