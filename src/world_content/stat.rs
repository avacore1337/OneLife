use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::icon::{Icon, IconType};
use crate::input::stat::{StatTypes, STAT_SIZE};
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize, Clone)]
pub struct Stat {
    pub name: StatTypes,
    pub description: &'static str,
    pub effect_description: &'static str,
    pub display_name: &'static str,
    pub required_tier: u32,
    pub icon: Icon,
}

impl Stat {
    pub fn get_stats_gains(&self, game: &mut Game) {
        let stat_state = &mut game.state.stats[self.name as usize];
        let level = stat_state.level;
        if !stat_state.is_visible {
            return;
        }

        let inter = &mut game.intermediate_state;
        match self.name {
            StatTypes::Con => {
                inter.add_multiplier(KeyValues::Health, 0.05 * level, self.display_name);
            }
            StatTypes::Int => {
                inter.add_multiplier(KeyValues::Stats, 1.0 + (0.1 * level), self.display_name);
                inter.add_multiplier(KeyValues::Skills, 1.0 + (0.1 * level), self.display_name);
            }
            StatTypes::Str => {
                inter.add_multiplier(KeyValues::SoldierXp, 0.05 * level, self.display_name);
            }
            StatTypes::Cha => {
                inter.add_multiplier(KeyValues::Coins, 1.0 + (0.05 * level), self.display_name);
            }
            StatTypes::Faith => {
                inter.add_multiplier(
                    KeyValues::DivineFavor,
                    1.0 + (0.05 * level),
                    self.display_name,
                );
            }
            _ => {}
        }
    }
}

pub fn translate_stat(stat: StatTypes) -> Stat {
    match stat {
        StatTypes::Str => Stat {
            name: stat,
            description: "Do you even lift?",
            effect_description: "Boosts income from Soldier jobs by 1 + level*0.1\nBoosts Soldier jobs XP by 1 + level*0.05",
            display_name: "Strength",
            required_tier: 2,
            icon: IconType::Str.into(),
        },
        StatTypes::Cha => Stat {
            name: stat,
            description: "Being liked has its benifits.",
            effect_description: "Boosts Coin gain by 1 + level*0.05",
            display_name: "Charisma",
            required_tier: 1,
            icon: IconType::Cha.into(),
        },
        StatTypes::Dex => Stat {
            name: stat,
            description: "Bendy!",
            effect_description: "todo",
            display_name: "Dexterity",
            required_tier: 7,
            icon: IconType::Dex.into(),
        },
        StatTypes::Int => Stat {
            name: stat,
            description: "Nerd!",
            effect_description: "Boosts XP gain for skills/stats by 1 + level*0.05",
            display_name: "Intelligence",
            required_tier: 0,
            icon: IconType::Int.into(),
        },
        StatTypes::Con => Stat {
            name: stat,
            description: "Endure the pain!",
            effect_description: "Boosts income from Labor jobs by 1 + level*0.1\nIncreases health gain by level*0.05",
            display_name: "Constitution",
            required_tier: 0,
            icon: IconType::Con.into(),
        },
        StatTypes::Faith => Stat {
            name: stat,
            description: "Have some.",
            effect_description: "Boosts income from Priest jobs by 1 + level*0.1\nBoosts Divine Favor gain by 1 + level*0.05",
            display_name: "Faith",
            required_tier: 4,
            icon: IconType::Faith.into(),
        },
    }
}

pub fn should_be_visible_stat(input_stat: StatTypes, game: &Game) -> bool {
    let stat = &game.world.stats[input_stat as usize];
    if stat.required_tier > game.state.rebirth_stats.tier {
        return false;
    }
    match input_stat {
        StatTypes::Faith => game.state.rebirth_stats.unlocks.has_faith,
        _ => true,
    }
}

pub fn get_stats() -> [Stat; STAT_SIZE] {
    let mut stats: [MaybeUninit<Stat>; STAT_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in StatTypes::iter() {
        stats[name as usize].write(translate_stat(name));
    }
    unsafe { mem::transmute(stats) }
}
