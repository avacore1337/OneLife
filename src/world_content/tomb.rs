// use crate::engine::intermediate_state::{Gain, IntermediateState};
// use crate::engine::value_keys::KeyValues;
// use crate::game::Game;
// use crate::input::stat::StatTypes;
use crate::input::tomb::{TombTypes, TOMB_SIZE};
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub struct Tomb {
    pub name: TombTypes,
    pub purchasing_cost: f64,
    pub description: &'static str,
    pub display_name: &'static str,
    pub required_tier: u32,
}

// impl Gain for Tomb {
//     fn gain(&self, intermediate: &mut IntermediateState) {
//         match self.name {
//             TombTypes::Mines => {
//                 intermediate.set_base(KeyValues::Health, -5.0);
//             }
//             TombTypes::Latrine => {
//                 intermediate.set_base(KeyValues::Health, -4.0);
//             }
//             TombTypes::GalleyRower => {
//                 intermediate.set_base(KeyValues::Health, -3.0);
//             }
//             TombTypes::Fields => {
//                 intermediate.set_base(KeyValues::Health, -2.0);
//             }
//             TombTypes::Mill => {
//                 intermediate.set_base(KeyValues::Health, -1.0);
//             }
//             _ => (),
//         }
//     }
// }
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
            purchasing_cost: 100_000.0,
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

// pub fn should_unlock_tomb(input_tomb: TombTypes, game: &Game) -> bool {
//     let tomb = &game.world.tombs[input_tomb as usize];
//     if tomb.required_tier > game.state.rebirth_stats.class_tier {
//         return false;
//     }
//     match input_tomb {
//         TombTypes::Mines => true,
//         TombTypes::Latrine => game.state.tombs[TombTypes::Mines as usize].level >= 10,
//         TombTypes::GalleyRower => game.state.tombs[TombTypes::Latrine as usize].level >= 10,
//         TombTypes::Fields => game.state.tombs[TombTypes::GalleyRower as usize].level >= 10,
//         TombTypes::Mill => game.state.tombs[TombTypes::Fields as usize].level >= 10,
//         TombTypes::Weaver => game.state.tombs[TombTypes::Mill as usize].level >= 10,
//         TombTypes::Fisherman => game.state.tombs[TombTypes::Weaver as usize].level >= 10,
//         TombTypes::Farmer => game.state.tombs[TombTypes::Fisherman as usize].level >= 10,
//     }
// }

// pub fn should_be_visable_tomb(input_tomb: TombTypes, game: &Game) -> bool {
//     let tomb = &game.world.tombs[input_tomb as usize];
//     tomb.required_tier <= game.state.rebirth_stats.class_tier + 1
// }

pub fn get_tombs() -> [Tomb; TOMB_SIZE] {
    let mut tombs: [MaybeUninit<Tomb>; TOMB_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in TombTypes::iter() {
        tombs[name as usize].write(translate_tomb(name));
    }
    unsafe { mem::transmute(tombs) }
}
