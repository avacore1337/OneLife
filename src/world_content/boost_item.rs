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
    pub effect_description: &'static str,
    pub display_name: &'static str,
    pub required_tier: u32,
}

impl Gain for BoostItem {
    fn gain(&self, inter: &mut IntermediateState) {
        match self.name {
            BoostItemTypes::Book => {
                inter.add_multiplier(KeyValues::Int, 1.5, self.display_name);
            }
            BoostItemTypes::Shoe1 => {
                inter.add_multiplier(KeyValues::Con, 1.5, self.display_name);
            }
            BoostItemTypes::Clothes1 => {
                inter.add_multiplier(KeyValues::Happiness, 1.5, self.display_name);
            }
            // BoostItemTypes::IronPickAxe => {
            //     inter.add_multiplier(KeyValues::Mines, 2.0, self.display_name);
            // }
            BoostItemTypes::IronAxe => {
                inter.add_multiplier(KeyValues::Woodcutter, 2.0, self.display_name);
            }
            BoostItemTypes::Book2 => {
                inter.add_multiplier(KeyValues::Int, 1.5, self.display_name);
            }
            BoostItemTypes::Shoe2 => {
                inter.add_multiplier(KeyValues::Con, 1.5, self.display_name);
            }
            BoostItemTypes::Clothes2 => {
                inter.add_multiplier(KeyValues::Happiness, 1.3, self.display_name);
            }
            BoostItemTypes::Book3 => {
                inter.add_multiplier(KeyValues::Int, 1.5, self.display_name);
            }
            BoostItemTypes::Book4 => {
                inter.add_multiplier(KeyValues::Int, 2.0, self.display_name);
            }
            BoostItemTypes::Book5 => {
                inter.add_multiplier(KeyValues::Int, 1.5, self.display_name);
            }
            BoostItemTypes::Shoe3 => {
                inter.add_multiplier(KeyValues::Con, 1.5, self.display_name);
            }
            BoostItemTypes::Shoe4 => {
                inter.add_multiplier(KeyValues::Con, 2.0, self.display_name);
            }
            BoostItemTypes::Shoe5 => {
                inter.add_multiplier(KeyValues::Con, 1.5, self.display_name);
            }
            BoostItemTypes::FishingGear => {
                inter.add_multiplier(KeyValues::Fisherman, 1.5, self.display_name);
            }
            BoostItemTypes::Clothes3 => {
                inter.add_multiplier(KeyValues::Happiness, 1.3, self.display_name);
            }
            BoostItemTypes::Clothes4 => {
                inter.add_multiplier(KeyValues::Happiness, 1.3, self.display_name);
            }
            BoostItemTypes::Clothes5 => {
                inter.add_multiplier(KeyValues::Happiness, 1.3, self.display_name);
            }
            BoostItemTypes::Leach => {
                inter.add_base(KeyValues::Health, 1.0, self.display_name);
            }
            BoostItemTypes::PitchFork => {
                inter.add_multiplier(KeyValues::Farmer, 2.0, self.display_name);
            }
            BoostItemTypes::HealthKit => {
                inter.add_base(KeyValues::Health, 2.0, self.display_name);
            }
            BoostItemTypes::Dumbell => {
                inter.add_multiplier(KeyValues::Str, 4.0, self.display_name);
            }
            BoostItemTypes::Dumbell2 => {
                inter.add_multiplier(KeyValues::Str, 2.0, self.display_name);
            }
            BoostItemTypes::Dumbell3 => {
                inter.add_multiplier(KeyValues::Str, 2.0, self.display_name);
            }
            BoostItemTypes::Dumbell4 => {
                inter.add_multiplier(KeyValues::Str, 2.0, self.display_name);
            }
            BoostItemTypes::Flower1 => {
                inter.add_multiplier(KeyValues::Cha, 2.0, self.display_name);
            }
            BoostItemTypes::Flower2 => {
                inter.add_multiplier(KeyValues::Cha, 2.0, self.display_name);
            }
            BoostItemTypes::Flower3 => {
                inter.add_multiplier(KeyValues::Cha, 2.0, self.display_name);
            }
            BoostItemTypes::Flower4 => {
                inter.add_multiplier(KeyValues::Cha, 2.0, self.display_name);
            }
            BoostItemTypes::Meditation1 => {
                inter.add_multiplier(KeyValues::Mindfull, 2.0, self.display_name);
            }
            BoostItemTypes::Meditation2 => {
                inter.add_multiplier(KeyValues::Mindfull, 2.0, self.display_name);
            }
            BoostItemTypes::Meditation3 => {
                inter.add_multiplier(KeyValues::Mindfull, 2.0, self.display_name);
            }
            BoostItemTypes::Meditation4 => {
                inter.add_multiplier(KeyValues::Mindfull, 2.0, self.display_name);
            }
            BoostItemTypes::Burial1 => {
                inter.add_multiplier(KeyValues::Coins, 2.0, self.display_name);
            }
            BoostItemTypes::Burial2 => {
                inter.add_multiplier(KeyValues::Coins, 2.0, self.display_name);
            }
            BoostItemTypes::Burial3 => {
                inter.add_multiplier(KeyValues::Coins, 2.0, self.display_name);
            }
            BoostItemTypes::Burial4 => {
                inter.add_multiplier(KeyValues::Coins, 2.0, self.display_name);
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
            effect_description: "2x Intelligence XP gain",
            display_name: "Book: Learning to read",
            required_tier: 0,
        },
        BoostItemTypes::Shoe1 => BoostItem {
            name: item_type,
            purchasing_cost: 1_000.0,
            description: "\"Shoes\"",
            effect_description: "2x Constitution XP gain",
            display_name: "Raggs Shoes",
            required_tier: 0,
        },
        BoostItemTypes::Clothes1 => BoostItem {
            name: item_type,
            purchasing_cost: 1_000.0,
            description: "You now actually have something to wear",
            effect_description: "1.5x Happiness",
            display_name: "Ragged Clothes",
            required_tier: 0,
        },
        // BoostItemTypes::IronPickAxe => BoostItem {
        //     name: item_type,
        //     purchasing_cost: 1_000.0,
        //     description: "Mining now goes faster",
        //     effect_description: "2x Mining income",
        //     display_name: "Iron Pickaxe",
        //     required_tier: 0,
        // },
        BoostItemTypes::Book2 => BoostItem {
            name: item_type,
            purchasing_cost: 4_000.0,
            description: "Very expensive for a childrens book",
            effect_description: "2x Intelligence XP gain",
            display_name: "Book: Reading for children",
            required_tier: 0,
        },
        BoostItemTypes::Shoe2 => BoostItem {
            name: item_type,
            purchasing_cost: 6_000.0,
            description: "Nice description of a piece of wood tied to you foot",
            effect_description: "2x Constitution XP gain",
            display_name: "Wooden Shoe",
            required_tier: 0,
        },

        // TOMB 1 - 10 000
        BoostItemTypes::Leach => BoostItem {
            name: item_type,
            purchasing_cost: 12_000.0,
            description: "Blood sucking bug, how is this healthy?",
            effect_description: "Improving Health",
            display_name: "Blood Leach",
            required_tier: 1,
        },
        BoostItemTypes::Clothes2 => BoostItem {
            name: item_type,
            purchasing_cost: 15_000.0,
            description: "Ragged but sturdy",
            effect_description: "1.5x Happiness",
            display_name: "Farmers Clothes",
            required_tier: 0,
        },
        BoostItemTypes::Shoe3 => BoostItem {
            name: item_type,
            purchasing_cost: 16_000.0,
            description: "Actually a type of shoe",
            effect_description: "2x Constitution XP gain",
            display_name: "Sandals",
            required_tier: 1,
        },
        BoostItemTypes::FishingGear => BoostItem {
            name: item_type,
            purchasing_cost: 16_000.0,
            description: "The salesman for this got you hook, line and sinker",
            effect_description: "2x Fisherman income",
            display_name: "Fishing Gear",
            required_tier: 1,
        },
        BoostItemTypes::Burial1 => BoostItem {
            name: item_type,
            purchasing_cost: 30_000.0,
            description: "todo",
            effect_description: "1.3x Coin gain",
            display_name: "Burial1",
            required_tier: 1,
        },
        BoostItemTypes::Book3 => BoostItem {
            name: item_type,
            purchasing_cost: 32_000.0,
            description: "The book makes you feel really really smart",
            effect_description: "2x Intelligence XP gain",
            display_name: "Book: Dunner Kruger's peak",
            required_tier: 1,
        },
        BoostItemTypes::Clothes3 => BoostItem {
            name: item_type,
            purchasing_cost: 40_000.0,
            description: "You fit into the city the less nice parts that is",
            effect_description: "1.5x Happiness",
            display_name: "City Clothes",
            required_tier: 1,
        },
        BoostItemTypes::Flower1 => BoostItem {
            name: item_type,
            purchasing_cost: 50_000.0,
            description: "Someone told you that flowers make great gifts",
            effect_description: "2x Charisma XP gain",
            display_name: "Roadside flower",
            required_tier: 1,
        },

        // TOMB 2 - 100 000
        BoostItemTypes::Dumbell => BoostItem {
            name: item_type,
            purchasing_cost: 2_500.0,
            description: "It's just a stick",
            effect_description: "2x Strength XP gain",
            display_name: "Wooden Dumbell",
            required_tier: 2,
        },
        BoostItemTypes::Dumbell2 => BoostItem {
            name: item_type,
            purchasing_cost: 25_000.0,
            description: "Couldn't I just use a normal stone?",
            effect_description: "2x Strength XP gain",
            display_name: "Stone dumbell",
            required_tier: 2,
        },
        BoostItemTypes::PitchFork => BoostItem {
            name: item_type,
            purchasing_cost: 64_000.0,
            description: "Grab one",
            effect_description: "2x Farmer income",
            display_name: "Pitchfork",
            required_tier: 2,
        },
        BoostItemTypes::Book4 => BoostItem {
            name: item_type,
            purchasing_cost: 320_000.0,
            description: "Back to the basics",
            effect_description: "2x Intelligence XP gain",
            display_name: "Book: Elementary Education",
            required_tier: 2,
        },
        BoostItemTypes::Shoe4 => BoostItem {
            name: item_type,
            purchasing_cost: 160_000.0,
            description: "Hey, these kind of actually fits your feet",
            effect_description: "2x Constitution XP gain",
            display_name: "Leather Sandals",
            required_tier: 2,
        },
        BoostItemTypes::Dumbell3 => BoostItem {
            name: item_type,
            purchasing_cost: 250_000.0,
            description: "Cowbell, Kettlebell, what's the difference anyway",
            effect_description: "2x Strength XP gain",
            display_name: "Kettlebell",
            required_tier: 2,
        },
        BoostItemTypes::Burial2 => BoostItem {
            name: item_type,
            purchasing_cost: 300_000.0,
            description: "todo",
            effect_description: "1.3x Coin gain",
            display_name: "Burial2",
            required_tier: 2,
        },
        BoostItemTypes::Clothes4 => BoostItem {
            name: item_type,
            purchasing_cost: 400_000.0,
            description: "You fit into the city the less nice parts that is",
            effect_description: "1.5x Happiness",
            display_name: "City Clothes",
            required_tier: 2,
        },
        BoostItemTypes::Flower2 => BoostItem {
            name: item_type,
            purchasing_cost: 500_000.0,
            description: "Some flowers are prettier than others",
            effect_description: "2x Charisma XP gain",
            display_name: "Daisy",
            required_tier: 2,
        },

        // TOMB 3 - 1 000 000
        BoostItemTypes::IronAxe => BoostItem {
            name: item_type,
            purchasing_cost: 800_000.0,
            description: "I sleep all night and I work all day",
            effect_description: "2x Woodcutter income",
            display_name: "Iron Axe",
            required_tier: 3,
        },
        BoostItemTypes::HealthKit => BoostItem {
            name: item_type,
            purchasing_cost: 1_200_000.0,
            description: "What is this fancy box of cloth",
            effect_description: "Improving Health",
            display_name: "Healthkit",
            required_tier: 3,
        },
        BoostItemTypes::Shoe5 => BoostItem {
            name: item_type,
            purchasing_cost: 1.6e6,
            description: "todo",
            effect_description: "2x Constitution XP gain",
            display_name: "Leather Sandals",
            required_tier: 3,
        },
        BoostItemTypes::Dumbell4 => BoostItem {
            name: item_type,
            purchasing_cost: 2.5e6,
            description: "Cowbell, Kettlebell, what's the difference anyway",
            effect_description: "2x Strength XP gain",
            display_name: "Kettlebell",
            required_tier: 3,
        },
        BoostItemTypes::Burial3 => BoostItem {
            name: item_type,
            purchasing_cost: 3.0e6,
            description: "todo",
            effect_description: "1.3x Coin gain",
            display_name: "Burial3",
            required_tier: 3,
        },
        BoostItemTypes::Book5 => BoostItem {
            name: item_type,
            purchasing_cost: 3.2e6,
            description: "todo",
            effect_description: "2x Intelligence XP gain",
            display_name: "Book: Elementary Education",
            required_tier: 3,
        },
        BoostItemTypes::Clothes5 => BoostItem {
            name: item_type,
            purchasing_cost: 4.0e6,
            description: "todo",
            effect_description: "1.5x Happiness",
            display_name: "City Clothes",
            required_tier: 3,
        },
        BoostItemTypes::Flower3 => BoostItem {
            name: item_type,
            purchasing_cost: 5.0e6,
            description: "Ok, this flower is way better",
            effect_description: "2x Charisma XP gain",
            display_name: "Tulip",
            required_tier: 3,
        },

        // TOMB 4 - 10 000 000
        BoostItemTypes::Burial4 => BoostItem {
            name: item_type,
            purchasing_cost: 30.0e6,
            description: "todo",
            effect_description: "1.3x Coin gain",
            display_name: "Burial4",
            required_tier: 3,
        },
        BoostItemTypes::Flower4 => BoostItem {
            name: item_type,
            purchasing_cost: 50.0e6,
            description: "todo",
            effect_description: "2x Charisma XP gain",
            display_name: "Tulip",
            required_tier: 4,
        },

        // TBD
        BoostItemTypes::Meditation1 => BoostItem {
            name: item_type,
            purchasing_cost: 8_000.0,
            description: "Made by someone named Yoga, weird name",
            effect_description: "2x Mindfullness XP gain",
            display_name: "Meditation Mat",
            required_tier: 5,
        },
        BoostItemTypes::Meditation2 => BoostItem {
            name: item_type,
            purchasing_cost: 80_000.0,
            description: "Smells like a hermit",
            effect_description: "2x Mindfullness XP gain",
            display_name: "Incense",
            required_tier: 5,
        },
        BoostItemTypes::Meditation3 => BoostItem {
            name: item_type,
            purchasing_cost: 800_000.0,
            description: "It's soft to sit on",
            effect_description: "2x Mindfullness XP gain",
            display_name: "Meditation Pillow",
            required_tier: 5,
        },
        BoostItemTypes::Meditation4 => BoostItem {
            name: item_type,
            purchasing_cost: 8.0e6,
            description: "todo",
            effect_description: "2x Mindfullness XP gain",
            display_name: "Meditation Pillow",
            required_tier: 5,
        },
    }
}

pub fn should_unlock_boost_item(input_boost_item: BoostItemTypes, game: &Game) -> bool {
    let boost_item = &game.world.boost_items[input_boost_item as usize];
    if boost_item.required_tier > game.state.rebirth_stats.tier {
        return false;
    }
    if boost_item.purchasing_cost > game.state.items.money {
        return false;
    }
    true
}

pub fn should_be_visible_boost_item(input_boost_item: BoostItemTypes, game: &Game) -> bool {
    let boost_item = &game.world.boost_items[input_boost_item as usize];
    let boost_item_state = &game.state.boost_items[input_boost_item as usize];
    if boost_item.required_tier > game.state.rebirth_stats.tier {
        return false;
    }
    if boost_item_state.is_visible {
        return true;
    }
    if boost_item.purchasing_cost / 20.0 > game.state.items.money {
        return false;
    }
    match input_boost_item {
        // BoostItemTypes::IronPickAxe => game.state.works[WorkTypes::Mines as usize].level > 25,
        BoostItemTypes::Clothes2 => {
            game.state.boost_items[BoostItemTypes::Clothes1 as usize].is_purchased
        }
        BoostItemTypes::Clothes3 => {
            game.state.boost_items[BoostItemTypes::Clothes2 as usize].is_purchased
        }
        BoostItemTypes::Clothes4 => {
            game.state.boost_items[BoostItemTypes::Clothes3 as usize].is_purchased
        }
        BoostItemTypes::FishingGear => game.state.works[WorkTypes::Fisherman as usize].level > 25,
        BoostItemTypes::PitchFork => game.state.works[WorkTypes::Farmer as usize].level > 25,
        BoostItemTypes::IronAxe => game.state.works[WorkTypes::Woodcutter as usize].level > 25,
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
