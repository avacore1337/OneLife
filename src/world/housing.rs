use crate::input::housing::Housing as InputHousing;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize)]
pub struct Housing {
    pub name: InputHousing,
    pub upkeep: f64,
    pub description: String,
    pub required_money: f64,
    pub happiness_factor: f64,
    pub health_effect: f64,
}

pub fn translate_housing(housing: &InputHousing) -> Housing {
    match housing {
        InputHousing::StoneFloor => Housing {
            name: InputHousing::StoneFloor,
            upkeep: 0.0,
            description: "You sleep where you can.".to_string(),
            required_money: 0.0,
            happiness_factor: 1.0,
            health_effect: -10.0,
        },
        InputHousing::ComfortableSpot => Housing {
            name: InputHousing::ComfortableSpot,
            upkeep: 1.0,
            description: "You pay some thugs to get to sleep in the \"nice\" corner.".to_string(),
            required_money: 0.0,
            happiness_factor: 1.5,
            health_effect: -3.0,
        },
        InputHousing::FilthyBarracks => Housing {
            name: InputHousing::FilthyBarracks,
            upkeep: 5.0,
            description: "Inside is better than outside. Probably.".to_string(),
            required_money: 100.0,
            happiness_factor: 2.0,
            health_effect: -1.0,
        },
        InputHousing::CrampedBarracks => Housing {
            name: InputHousing::CrampedBarracks,
            upkeep: 15.0,
            description: "Your room mates now outnumber the rats".to_string(),
            required_money: 1000.0,
            happiness_factor: 4.0,
            health_effect: 0.0,
        },
    }
}

pub fn get_housing() -> Vec<Housing> {
    let mut housing = Vec::<Housing>::new();
    for input_housing in InputHousing::iter() {
        housing.push(translate_housing(&input_housing));
    }
    housing
}
