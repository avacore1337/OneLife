use crate::engine::intermediate_state::{Gain, IntermediateState};
use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::input::boost_item::{BoostItemTypes, BOOST_ITEM_SIZE};
use crate::input::work::WorkTypes;
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
}

impl Gain for BoostItem {
    fn gain(&self, intermediate: &mut IntermediateState) {
        match self.name {
            BoostItemTypes::Book => {
                intermediate.add_multiplier(KeyValues::Int, 1.5, "Book");
            }
            BoostItemTypes::Shoe1 => {
                intermediate.add_multiplier(KeyValues::Con, 1.5, "Shoe");
            }
            BoostItemTypes::RaggedClothes => {
                intermediate.add_multiplier(KeyValues::Happiness, 1.5, "Ragged Clothes");
            }
            BoostItemTypes::IronPickAxe => {
                intermediate.add_multiplier(KeyValues::Mines, 2.0, "Mines");
            }
            BoostItemTypes::Book2 => {
                intermediate.add_multiplier(KeyValues::Int, 2.0, "Book");
            }
            BoostItemTypes::Shoe2 => {
                intermediate.add_multiplier(KeyValues::Con, 2.0, "Shoe");
            }
            BoostItemTypes::FarmersClothes => {
                intermediate.add_multiplier(KeyValues::Happiness, 1.5, "Farmers Clothes");
            }
            BoostItemTypes::Book3 => {
                intermediate.add_multiplier(KeyValues::Int, 2.0, "Book");
            }
            BoostItemTypes::Shoe3 => {
                intermediate.add_multiplier(KeyValues::Con, 2.0, "Shoe");
            }
            BoostItemTypes::FishingGear => {
                intermediate.add_multiplier(KeyValues::Fisherman, 2.0, "Fishing Gear");
            }
            BoostItemTypes::CityClothes => {
                intermediate.add_multiplier(KeyValues::Happiness, 1.5, "City Clothes");
            }
            BoostItemTypes::Leach => {
                intermediate.add_base(KeyValues::Health, 1.0);
            }
            BoostItemTypes::PitchFork => {
                intermediate.add_multiplier(KeyValues::Farmer, 2.0, "PitchFork");
            }
            BoostItemTypes::HealthKit => {
                intermediate.add_base(KeyValues::Health, 2.0);
            }
            BoostItemTypes::Dumbell => {
                intermediate.add_multiplier(KeyValues::Str, 1.5, "Dumbell");
            }
            BoostItemTypes::Dumbell2 => {
                intermediate.add_multiplier(KeyValues::Str, 2.0, "Dumbell");
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
            purchasing_cost: 100.0,
            description: "Me dumb? That's unpossible",
            display_name: "Learning to read",
            required_tier: 0,
        },
        BoostItemTypes::Shoe1 => BoostItem {
            name: item_type,
            purchasing_cost: 1_000.0,
            description: "\"Shoes\"",
            display_name: "Raggs Shoes",
            required_tier: 0,
        },
        BoostItemTypes::RaggedClothes => BoostItem {
            name: item_type,
            purchasing_cost: 1_000.0,
            description: "You now actually have something to wear",
            display_name: "Ragged Clothes",
            required_tier: 0,
        },
        BoostItemTypes::IronPickAxe => BoostItem {
            name: item_type,
            purchasing_cost: 1_000.0,
            description: "Mining now goes faster",
            display_name: "Iron Pickaxe",
            required_tier: 0,
        },
        BoostItemTypes::Book2 => BoostItem {
            name: item_type,
            purchasing_cost: 4_000.0,
            description: "Very expensive for a childrens book",
            display_name: "Reading for children",
            required_tier: 0,
        },
        BoostItemTypes::Shoe2 => BoostItem {
            name: item_type,
            purchasing_cost: 6_000.0,
            description: "Nice description of a piece of wood tied to you foot",
            display_name: "Wooden Shoe",
            required_tier: 0,
        },
        BoostItemTypes::FarmersClothes => BoostItem {
            name: item_type,
            purchasing_cost: 15_000.0,
            description: "Ragged but sturdy",
            display_name: "Farmers Clothes",
            required_tier: 0,
        },
        BoostItemTypes::Book3 => BoostItem {
            name: item_type,
            purchasing_cost: 16_000.0,
            description: "The book makes you feel really really smart",
            display_name: "Dunner Kruger's peak",
            required_tier: 1,
        },
        BoostItemTypes::Shoe3 => BoostItem {
            name: item_type,
            purchasing_cost: 16_000.0,
            description: "Actually a type of shoe",
            display_name: "Sandals",
            required_tier: 1,
        },
        BoostItemTypes::FishingGear => BoostItem {
            name: item_type,
            purchasing_cost: 16_000.0,
            description: "The salesman for this got you hook, line and sinker",
            display_name: "Fishing Gear",
            required_tier: 1,
        },
        BoostItemTypes::CityClothes => BoostItem {
            name: item_type,
            purchasing_cost: 40_000.0,
            description: "You fit into the city the less nice parts that is",
            display_name: "City Clothes",
            required_tier: 1,
        },
        BoostItemTypes::Leach => BoostItem {
            name: item_type,
            purchasing_cost: 400_000.0,
            description: "Blood sucking bug, how is this healthy?",
            display_name: "Blood Leach",
            required_tier: 2,
        },
        BoostItemTypes::PitchFork => BoostItem {
            name: item_type,
            purchasing_cost: 800_000.0,
            description: "Blood sucking bug, how is this healthy?",
            display_name: "Pitchfork",
            required_tier: 2,
        },
        BoostItemTypes::Dumbell => BoostItem {
            name: item_type,
            purchasing_cost: 1_000_000.0,
            description: "It's just a stick",
            display_name: "Wooden Dumbell",
            required_tier: 2,
        },
        BoostItemTypes::Dumbell2 => BoostItem {
            name: item_type,
            purchasing_cost: 3_000_000.0,
            description: "Couldn't I just use a normal stone?",
            display_name: "Stone dumbell",
            required_tier: 2,
        },
        BoostItemTypes::HealthKit => BoostItem {
            name: item_type,
            purchasing_cost: 4_000_000.0,
            description: "What is this fancy box of cloth",
            display_name: "Healthkit",
            required_tier: 3,
        },
        BoostItemTypes::Dumbell3 => BoostItem {
            name: item_type,
            purchasing_cost: 16_000_000.0,
            description: "Cowbell, Kettlebell, what's the difference anyway",
            display_name: "Kettlebell",
            required_tier: 3,
        },
    }
}

pub fn should_unlock_boost_item(input_boost_item: BoostItemTypes, game: &Game) -> bool {
    let boost_item = &game.world.boost_items[input_boost_item as usize];
    if boost_item.required_tier > game.state.rebirth_stats.class_tier {
        return false;
    }
    if boost_item.purchasing_cost > game.state.items.money {
        return false;
    }
    true
}

pub fn should_be_visible_boost_item(input_boost_item: BoostItemTypes, game: &Game) -> bool {
    let boost_item = &game.world.boost_items[input_boost_item as usize];
    if boost_item.required_tier > game.state.rebirth_stats.class_tier {
        return false;
    }
    match input_boost_item {
        BoostItemTypes::IronPickAxe => game.state.works[WorkTypes::Mines as usize].level > 25,
        BoostItemTypes::FarmersClothes => {
            game.state.items.boost_items[BoostItemTypes::RaggedClothes as usize].is_purchased
        }
        BoostItemTypes::CityClothes => {
            game.state.items.boost_items[BoostItemTypes::FarmersClothes as usize].is_purchased
        }
        BoostItemTypes::FishingGear => game.state.works[WorkTypes::Fisherman as usize].level > 25,
        BoostItemTypes::PitchFork => game.state.works[WorkTypes::Farmer as usize].level > 25,
        _ => true,
    }
}

pub fn get_boost_items() -> [BoostItem; BOOST_ITEM_SIZE] {
    let mut boost_items: [MaybeUninit<BoostItem>; BOOST_ITEM_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in BoostItemTypes::iter() {
        boost_items[name as usize].write(translate_boost_item(name));
    }
    unsafe { mem::transmute(boost_items) }
}
