use crate::engine::intermediate_state::{Gain, IntermediateState};
use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::input::tomb::{TombTypes, TOMB_SIZE};
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize, Clone)]
pub struct Tomb {
    pub name: TombTypes,
    pub purchasing_cost: f64,
    pub description: &'static str,
    pub display_name: &'static str,
    pub required_tier: u32,
}

impl Gain for Tomb {
    fn gain(&self, intermediate: &mut IntermediateState) {
        match self.name {
            TombTypes::Nothing => {
                intermediate.set_base(KeyValues::Coins, 0.0);
            }
            TombTypes::ShallowGrave => {
                intermediate.set_base(KeyValues::Coins, 2.0);
            }
            TombTypes::Tomb => {
                intermediate.set_base(KeyValues::Coins, 16.0);
            }
            TombTypes::Mausuleum => {
                intermediate.set_base(KeyValues::Coins, 320.0);
            }
        }
    }
}

pub const fn translate_tomb(tomb: TombTypes) -> Tomb {
    match tomb {
        TombTypes::Nothing => Tomb {
            name: tomb,
            purchasing_cost: 0.0,
            description: "You won't like what happens when you die...",
            display_name: "None",
            required_tier: 0,
        },
        TombTypes::ShallowGrave => Tomb {
            name: tomb,
            purchasing_cost: 10_000.0,
            description: "It's not honorable, but it means something",
            display_name: "Shallow Grave",
            required_tier: 0,
        },
        TombTypes::Tomb => Tomb {
            name: tomb,
            purchasing_cost: 1_000_000.0,
            description: "You were someone",
            display_name: "Tomb ",
            required_tier: 1,
        },
        TombTypes::Mausuleum => Tomb {
            name: tomb,
            purchasing_cost: 1_000_000_000.0,
            description: "A man of importance!",
            display_name: "Mausuleum ",
            required_tier: 4,
        },
    }
}

pub fn should_unlock_tomb(input_tomb: TombTypes, game: &Game) -> bool {
    let tomb = &game.world.tombs[input_tomb as usize];
    if tomb.required_tier > game.state.rebirth_stats.class_tier {
        return false;
    }
    if tomb.purchasing_cost > game.state.rebirth_stats.coins {
        return false;
    }
    true
    // match input_tomb {
    //     TombTypes::Mines => true,
    //     TombTypes::Latrine => game.state.tombs[TombTypes::Mines as usize].level >= 10,
    // }
}

pub fn should_be_visible_tomb(input_tomb: TombTypes, game: &Game) -> bool {
    let tomb = &game.world.tombs[input_tomb as usize];
    tomb.required_tier <= game.state.rebirth_stats.class_tier + 1
}

pub fn get_tombs() -> [Tomb; TOMB_SIZE] {
    let mut tombs: [MaybeUninit<Tomb>; TOMB_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in TombTypes::iter() {
        tombs[name as usize].write(translate_tomb(name));
    }
    unsafe { mem::transmute(tombs) }
}
