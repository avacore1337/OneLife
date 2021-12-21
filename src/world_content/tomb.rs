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
    pub coin_gain: f64,
}

impl Gain for Tomb {
    fn gain(&self, intermediate: &mut IntermediateState) {
        intermediate.set_base(KeyValues::Coins, self.coin_gain);
    }
}

pub const fn translate_tomb(tomb: TombTypes) -> Tomb {
    match tomb {
        TombTypes::ShallowGrave => Tomb {
            name: tomb,
            purchasing_cost: 10_000.0,
            description: "It's not honorable, but it means something",
            display_name: "Shallow Grave",
            required_tier: 0,
            coin_gain: 2.0,
        },
        TombTypes::BurialPit => Tomb {
            name: tomb,
            purchasing_cost: 1.0e6,
            description: "todo",
            display_name: "BurialPit ",
            required_tier: 1,
            coin_gain: 16.0,
        },
        TombTypes::Grave => Tomb {
            name: tomb,
            purchasing_cost: 1.0e8,
            description: "You were someone",
            display_name: "Grave ",
            required_tier: 2,
            coin_gain: 64.0,
        },
        TombTypes::FamilyGrave => Tomb {
            name: tomb,
            purchasing_cost: 1.0e10,
            description: "Wait, who's family is burried here?",
            display_name: "FamilyGrave ",
            required_tier: 3,
            coin_gain: 320.0,
        },
        TombTypes::Tomb => Tomb {
            name: tomb,
            purchasing_cost: 1.0e12,
            description: "Hope you don't get raided",
            display_name: "Tomb ",
            required_tier: 4,
            coin_gain: 1000.0,
        },
        TombTypes::Crypt => Tomb {
            name: tomb,
            purchasing_cost: 1.0e14,
            description: "A mysterious place",
            display_name: "Crypt ",
            required_tier: 5,
            coin_gain: 4.0e3,
        },
        TombTypes::Mausuleum => Tomb {
            name: tomb,
            purchasing_cost: 1.0e16,
            description: "Why are there so many mice in here?",
            display_name: "Mausuleum ",
            required_tier: 6,
            coin_gain: 16.0e3,
        },
        TombTypes::Catacomb => Tomb {
            name: tomb,
            purchasing_cost: 1.0e18,
            description: "This one has cats to take care of the mice",
            display_name: "Catacomb ",
            required_tier: 7,
            coin_gain: 64.0e3,
        },
    }
}

pub fn should_unlock_tomb(input_tomb: TombTypes, game: &Game) -> bool {
    let tomb = &game.world.tombs[input_tomb as usize];
    if tomb.required_tier > game.state.rebirth_stats.class_tier {
        return false;
    }
    if tomb.purchasing_cost > game.state.items.money {
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
