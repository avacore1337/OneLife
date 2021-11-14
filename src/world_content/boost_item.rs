// use crate::game::Game;
use crate::engine::intermediate_state::{Gain, IntermediateState};
use crate::engine::value_keys::KeyValues;
use crate::input::boost_item::BoostItemTypes;
use serde::Serialize;
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub struct BoostItem {
    pub name: BoostItemTypes,
    pub purchasing_cost: f64,
    pub description: &'static str,
    pub required_tier: u32,
    // pub item_effect:
}

impl Gain for BoostItem {
    fn gain(&self, intermediate: &mut IntermediateState) {
        match self.name {
            BoostItemTypes::Book => {
                intermediate.add_multiplier(KeyValues::Int, 2.0, "Book");
            }
            BoostItemTypes::Dumbell => {
                intermediate.add_multiplier(KeyValues::Str, 2.0, "Dumbell");
            }
        }
    }
}

pub fn translate_boost_item(item_type: BoostItemTypes) -> BoostItem {
    match item_type {
        BoostItemTypes::Book => BoostItem {
            name: BoostItemTypes::Book,
            purchasing_cost: 0.5,
            description: "Educationer",
            required_tier: 0,
        },
        BoostItemTypes::Dumbell => BoostItem {
            name: BoostItemTypes::Dumbell,
            purchasing_cost: 1_000_000.5,
            description: "Heavy thing",
            required_tier: 1,
        },
    }
}

// pub fn should_unlock_work(input_work: InputWork, game: &Game) -> bool {
//     let work = &game.world.works[input_work as usize];
//     if work.required_tier > game.state.rebirth_stats.class_tier {
//         return false;
//     }
//     match input_work {
//         InputWork::Mines => true,
//         InputWork::Fields => game.state.works[InputWork::Mines as usize].current_level > 10,
//         InputWork::Servant => game.state.works[InputWork::Fields as usize].current_level > 10,
//         InputWork::Teacher => game.state.works[InputWork::Servant as usize].current_level > 10,
//         InputWork::Farm => game.state.works[InputWork::Teacher as usize].current_level > 10,
//     }
// }

// pub fn should_be_visable_work(input_work: InputWork, game: &Game) -> bool {
//     let work = &game.world.works[input_work as usize];
//     work.required_tier <= game.state.rebirth_stats.class_tier + 1
// }

pub fn get_boost_items() -> Vec<BoostItem> {
    let mut boost_items = Vec::<BoostItem>::new();
    for boost_item_type in BoostItemTypes::iter() {
        boost_items.push(translate_boost_item(boost_item_type));
    }
    boost_items
}
