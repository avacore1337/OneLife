// use crate::game::Game;
use crate::engine::intermediate_state::{Gain, IntermediateState};
use crate::engine::value_keys::KeyValues;
use crate::input::boost_item::{BoostItemTypes, BOOST_ITEM_SIZE};
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub struct BoostItem {
    pub name: BoostItemTypes,
    pub purchasing_cost: f64,
    pub description: &'static str,
    pub display_name: &'static str,
    pub required_tier: u32,
    // pub item_effect:
}

impl Gain for BoostItem {
    fn gain(&self, intermediate: &mut IntermediateState) {
        match self.name {
            BoostItemTypes::Book => {
                intermediate.add_multiplier(KeyValues::Int, 1.5, "Book");
            }
            BoostItemTypes::Dumbell => {
                intermediate.add_multiplier(KeyValues::Str, 1.5, "Dumbell");
            }
            BoostItemTypes::Book2 => {
                intermediate.add_multiplier(KeyValues::Int, 2.0, "Book");
            }
            BoostItemTypes::Dumbell2 => {
                intermediate.add_multiplier(KeyValues::Str, 2.0, "Dumbell");
            }
            BoostItemTypes::Book3 => {
                intermediate.add_multiplier(KeyValues::Int, 2.0, "Book");
            }
            BoostItemTypes::Dumbell3 => {
                intermediate.add_multiplier(KeyValues::Str, 2.0, "Dumbell");
            }
        }
    }
}

pub fn translate_boost_item(item_type: BoostItemTypes) -> BoostItem {
    match item_type {
        BoostItemTypes::Book => BoostItem {
            name: item_type,
            purchasing_cost: 10.0,
            description: "Me dumb? That's unpossible",
            display_name: "Learning to read",
            required_tier: 0,
        },
        BoostItemTypes::Dumbell => BoostItem {
            name: item_type,
            purchasing_cost: 1_000.0,
            description: "It's just a stick",
            display_name: "Wooden Dumbell",
            required_tier: 0,
        },
        BoostItemTypes::Book2 => BoostItem {
            name: item_type,
            purchasing_cost: 4_000.0,
            description: "Very expensive for a childrens book",
            display_name: "Reading for children",
            required_tier: 0,
        },
        BoostItemTypes::Dumbell2 => BoostItem {
            name: item_type,
            purchasing_cost: 6_000.0,
            description: "Couldn't I just use a normal stone?",
            display_name: "Stone dumbell",
            required_tier: 0,
        },
        BoostItemTypes::Book3 => BoostItem {
            name: item_type,
            purchasing_cost: 16_000.0,
            description: "The book makes you feel really really smart",
            display_name: "Dunner Kruger's peak",
            required_tier: 1,
        },
        BoostItemTypes::Dumbell3 => BoostItem {
            name: item_type,
            purchasing_cost: 16_000.0,
            description: "Cowbell, Kettlebell, what's the difference anyway",
            display_name: "Kettlebell",
            required_tier: 1,
        },
    }
}

// pub fn should_unlock_work(input_work: WorkTypes, game: &Game) -> bool {
//     let work = &game.world.works[input_work as usize];
//     if work.required_tier > game.state.rebirth_stats.class_tier {
//         return false;
//     }
//     match input_work {
//         WorkTypes::Mines => true,
//         WorkTypes::Fields => game.state.works[WorkTypes::Mines as usize].level > 10,
//         WorkTypes::Servant => game.state.works[WorkTypes::Fields as usize].level > 10,
//         WorkTypes::Teacher => game.state.works[WorkTypes::Servant as usize].level > 10,
//         WorkTypes::Farm => game.state.works[WorkTypes::Teacher as usize].level > 10,
//     }
// }

// pub fn should_be_visible_work(input_work: WorkTypes, game: &Game) -> bool {
//     let work = &game.world.works[input_work as usize];
//     work.required_tier <= game.state.rebirth_stats.class_tier + 1
// }

pub fn get_boost_items() -> [BoostItem; BOOST_ITEM_SIZE] {
    let mut boost_items: [MaybeUninit<BoostItem>; BOOST_ITEM_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in BoostItemTypes::iter() {
        boost_items[name as usize].write(translate_boost_item(name));
    }
    unsafe { mem::transmute(boost_items) }
}
