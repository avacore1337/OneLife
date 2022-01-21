use crate::engine::intermediate_state::{Gain, IntermediateState};
use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::input::blessing::{BlessingTypes, BLESSING_SIZE};
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub struct Blessing {
    pub name: BlessingTypes,
    pub base_purchasing_cost: f64,
    pub description: &'static str,
    pub display_name: &'static str,
    pub required_tier: u32,
}

impl Gain for Blessing {
    fn gain(&self, intermediate: &mut IntermediateState) {
        match self.name {
            BlessingTypes::HeruclesStrength => {
                intermediate.add_multiplier(KeyValues::Str, 2.0, self.display_name);
            }
            BlessingTypes::AthenasWisdom => {
                intermediate.add_multiplier(KeyValues::Int, 2.0, self.display_name);
            }
        }
    }
}

pub const fn translate_blessing(blessing: BlessingTypes) -> Blessing {
    match blessing {
        BlessingTypes::HeruclesStrength => Blessing {
            name: blessing,
            base_purchasing_cost: 5.0,
            description: "He's a Hero!",
            display_name: "Herucles Strength",
            required_tier: 0,
        },
        BlessingTypes::AthenasWisdom => Blessing {
            name: blessing,
            base_purchasing_cost: 500.0,
            description: "The Wit!",
            display_name: "Athenas Wisdom",
            required_tier: 0,
        },
    }
}

pub fn calculate_blessing_next_level_cost(input_blessing: BlessingTypes, game: &Game) -> f64 {
    let blessing = &game.state.blessings[input_blessing as usize];
    let blessing_world = &game.world.blessings[input_blessing as usize];
    blessing_world.base_purchasing_cost * 1.1f64.powi(blessing.level as i32)
}

pub fn should_unlock_blessing(input_blessing: BlessingTypes, game: &Game) -> bool {
    let blessing = &game.state.blessings[input_blessing as usize];
    let blessing_world = &game.world.blessings[input_blessing as usize];
    if blessing_world.required_tier > game.state.rebirth_stats.tier {
        return false;
    }
    if blessing.next_level_cost > game.state.items.divine_favor {
        return false;
    }
    true
}

pub fn should_be_visible_blessing(input_blessing: BlessingTypes, game: &Game) -> bool {
    let blessing = &game.world.blessings[input_blessing as usize];
    blessing.required_tier <= game.state.rebirth_stats.tier
}

pub fn get_blessings() -> [Blessing; BLESSING_SIZE] {
    let mut blessings: [MaybeUninit<Blessing>; BLESSING_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in BlessingTypes::iter() {
        blessings[name as usize].write(translate_blessing(name));
    }
    unsafe { mem::transmute(blessings) }
}
