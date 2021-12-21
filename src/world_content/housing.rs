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
    pub description: String,
    pub display_name: &'static str,
    pub required_money: f64,
    pub happiness_factor: f64,
    pub health_effect: f64,
}

impl Gain for Housing {
    fn gain(&self, intermediate: &mut IntermediateState) {
        intermediate.add_multiplier(KeyValues::Happiness, self.happiness_factor, "housing");
    }
}

pub fn translate_housing(housing: HousingTypes) -> Housing {
    match housing {
        HousingTypes::StoneFloor => Housing {
            name: housing,
            upkeep: 0.0,
            description: "You sleep where you can.".to_string(),
            display_name: "Stone Floor",
            required_money: 0.0,
            happiness_factor: 1.0,
            health_effect: -10.0,
        },
        HousingTypes::ComfortableSpot => Housing {
            name: housing,
            upkeep: 1.0,
            description: "You pay some thugs to get to sleep in the \"nice\" corner.".to_string(),
            display_name: "A Comforable Spot",
            required_money: 0.0,
            happiness_factor: 1.5,
            health_effect: -3.0,
        },
        HousingTypes::FilthyBarracks => Housing {
            name: housing,
            upkeep: 3.0,
            description: "Inside is better than outside. Probably.".to_string(),
            display_name: "Filthy Barracks",
            required_money: 100.0,
            happiness_factor: 2.0,
            health_effect: -1.0,
        },
        HousingTypes::CrampedBarracks => Housing {
            name: housing,
            upkeep: 8.0,
            description: "Your room mates now outnumber the rats".to_string(),
            display_name: "Cramped Barracks",
            required_money: 1000.0,
            happiness_factor: 4.0,
            health_effect: 0.0,
        },
        HousingTypes::SharedRoom => Housing {
            name: housing,
            upkeep: 32.0,
            description: "Most of your room mates are sane. Most of them.".to_string(),
            display_name: "Shared Room",
            required_money: 10_000.0,
            happiness_factor: 8.0,
            health_effect: 0.0,
        },
        HousingTypes::LargeCloset => Housing {
            name: housing,
            upkeep: 128.0,
            description: "It sure ain't big, but it's your own space".to_string(),
            display_name: "Large Closet",
            required_money: 100_000.0,
            happiness_factor: 16.0,
            health_effect: 0.0,
        },
        HousingTypes::PrivateRoom => Housing {
            name: housing,
            upkeep: 500.0,
            description: "It almost feel weird not having people around you 24/7".to_string(),
            display_name: "Private Room",
            required_money: 1.0e6,
            happiness_factor: 32.0,
            health_effect: 1.0,
        },
        HousingTypes::TinyAppartment => Housing {
            name: housing,
            upkeep: 2_000.0,
            description: "It even includes your own \"kitchen\"".to_string(),
            display_name: "Tiny Appartment",
            required_money: 1.0e7,
            happiness_factor: 64.0,
            health_effect: 3.0,
        },
    }
}

pub fn should_be_visible_housing(input_housing: HousingTypes, game: &Game) -> bool {
    let housing = &game.world.housing[input_housing as usize];
    housing.required_money <= game.state.items.money
}

pub fn get_housings() -> [Housing; HOUSING_SIZE] {
    let mut housing: [MaybeUninit<Housing>; HOUSING_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in HousingTypes::iter() {
        housing[name as usize].write(translate_housing(name));
    }
    unsafe { mem::transmute(housing) }
}
