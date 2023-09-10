use crate::engine::intermediate_state::{Gain, IntermediateState};
use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::input::housing::{HousingTypes, HOUSING_SIZE};
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub struct Housing {
    pub name: HousingTypes,
    pub upkeep: f64,
    pub description: &'static str,
    pub display_name: &'static str,
    pub required_money: f64,
    pub happiness_factor: f64,
    pub health_effect: f64,
}

impl Gain for Housing {
    fn gain(&self, intermediate: &mut IntermediateState) {
        intermediate.add_multiplier(
            KeyValues::Happiness,
            self.happiness_factor,
            self.display_name,
        );
    }
}

pub fn translate_housing(housing: HousingTypes) -> Housing {
    match housing {
        HousingTypes::StoneFloor => Housing {
            name: housing,
            upkeep: 0.0,
            description: "You sleep where you can.",
            display_name: "Stone Floor",
            required_money: 0.0,
            happiness_factor: 1.0,
            health_effect: -10.0,
        },
        HousingTypes::ComfortableSpot => Housing {
            name: housing,
            upkeep: 1.0,
            description: "You pay some thugs to get to sleep in the \"nice\" corner.",
            display_name: "A Comforable Spot",
            required_money: 0.0,
            happiness_factor: 1.5,
            health_effect: -3.0,
        },
        HousingTypes::FilthyBarracks => Housing {
            name: housing,
            upkeep: 4.0,
            description: "Inside is better than outside. Probably.",
            display_name: "Filthy Barracks",
            required_money: 100.0,
            happiness_factor: 2.0,
            health_effect: -1.0,
        },
        HousingTypes::CrampedBarracks => Housing {
            name: housing,
            upkeep: 16.0,
            description: "Your room mates now outnumber the rats",
            display_name: "Cramped Barracks",
            required_money: 1000.0,
            happiness_factor: 4.0,
            health_effect: 0.0,
        },
        HousingTypes::SharedRoom => Housing {
            name: housing,
            upkeep: 64.0,
            description: "Most of your room mates are sane. Most of them.",
            display_name: "Shared Room",
            required_money: 4_000.0,
            happiness_factor: 6.0,
            health_effect: 0.0,
        },
        HousingTypes::LargeCloset => Housing {
            name: housing,
            upkeep: 256.0,
            description: "It sure ain't big, but it's your own space",
            display_name: "Large Closet",
            required_money: 16_000.0,
            happiness_factor: 9.0,
            health_effect: 0.0,
        },
        HousingTypes::PrivateRoom => Housing {
            name: housing,
            upkeep: 1000.0,
            description: "It almost feel weird not having people around you 24/7",
            display_name: "Private Room",
            required_money: 64_000.0,
            happiness_factor: 14.0,
            health_effect: 1.0,
        },
        HousingTypes::TinyApartment => Housing {
            name: housing,
            upkeep: 4_000.0,
            description: "It even includes your own \"kitchen\"",
            display_name: "Tiny Apartment",
            required_money: 256_000.0,
            happiness_factor: 20.0,
            health_effect: 2.0,
        },
        HousingTypes::SmallApartment => Housing {
            name: housing,
            upkeep: 16_000.0,
            description: "It even includes your own \"kitchen\"",
            display_name: "Small Apartment",
            required_money: 1.0e6,
            happiness_factor: 30.0,
            health_effect: 3.0,
        },
        HousingTypes::Apartment => Housing {
            name: housing,
            upkeep: 64_000.0,
            description: "It even includes your own \"kitchen\"",
            display_name: "Apartment",
            required_money: 4.0e6,
            happiness_factor: 45.0,
            health_effect: 4.0,
        },
    }
}

pub fn should_unlock_housing(input_housing: HousingTypes, game: &Game) -> bool {
    let housing = &game.world.get_housing(input_housing);
    let housing_state = &game.state.housing[input_housing as usize];
    housing.required_money <= game.state.items.money || housing_state.is_unlocked
}

pub fn should_be_visible_housing(input_housing: HousingTypes, game: &Game) -> bool {
    let housing = game.world.get_housing(input_housing);
    let housing_state = &game.state.housing[input_housing as usize];
    housing.required_money / 4.0 <= game.state.items.money || housing_state.is_visible
}

pub fn get_housings() -> [Housing; HOUSING_SIZE] {
    let mut housing: [MaybeUninit<Housing>; HOUSING_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in HousingTypes::iter() {
        housing[name as usize].write(translate_housing(name));
    }
    unsafe { mem::transmute(housing) }
}
