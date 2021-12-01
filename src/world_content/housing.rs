use crate::engine::intermediate_state::{Gain, IntermediateState};
use crate::engine::value_keys::KeyValues;
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
            name: HousingTypes::StoneFloor,
            upkeep: 0.0,
            description: "You sleep where you can.".to_string(),
            display_name: "Stone Floor",
            required_money: 0.0,
            happiness_factor: 1.0,
            health_effect: -10.0,
        },
        HousingTypes::ComfortableSpot => Housing {
            name: HousingTypes::ComfortableSpot,
            upkeep: 1.0,
            description: "You pay some thugs to get to sleep in the \"nice\" corner.".to_string(),
            display_name: "A Comforable Spot",
            required_money: 0.0,
            happiness_factor: 1.5,
            health_effect: -3.0,
        },
        HousingTypes::FilthyBarracks => Housing {
            name: HousingTypes::FilthyBarracks,
            upkeep: 3.0,
            description: "Inside is better than outside. Probably.".to_string(),
            display_name: "Filthy Barracks",
            required_money: 100.0,
            happiness_factor: 2.0,
            health_effect: -1.0,
        },
        HousingTypes::CrampedBarracks => Housing {
            name: HousingTypes::CrampedBarracks,
            upkeep: 8.0,
            description: "Your room mates now outnumber the rats".to_string(),
            display_name: "Cramped Barracks",
            required_money: 1000.0,
            happiness_factor: 4.0,
            health_effect: 0.0,
        },
    }
}

pub fn get_housings() -> [Housing; HOUSING_SIZE] {
    let mut housing: [MaybeUninit<Housing>; HOUSING_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in HousingTypes::iter() {
        housing[name as usize].write(translate_housing(name));
    }
    unsafe { mem::transmute(housing) }
}
