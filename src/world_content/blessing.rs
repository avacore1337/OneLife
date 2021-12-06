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
    pub purchasing_cost: f64,
    pub description: &'static str,
    pub display_name: &'static str,
    pub required_tier: u32,
}

impl Gain for Blessing {
    fn gain(&self, intermediate: &mut IntermediateState) {
        match self.name {
            BlessingTypes::HeruclesStrength => {
                intermediate.add_multiplier(KeyValues::Str, 2.0, "Herucles Strength");
            }
            BlessingTypes::AthenasWisdom => {
                intermediate.add_multiplier(KeyValues::Int, 2.0, "Athenas Wisdom");
            }
        }
    }
}

pub const fn translate_blessing(blessing: BlessingTypes) -> Blessing {
    match blessing {
        BlessingTypes::HeruclesStrength => Blessing {
            name: blessing,
            purchasing_cost: 5.0,
            description: "He's a Hero!",
            display_name: "Herucles Strength",
            required_tier: 0,
        },
        BlessingTypes::AthenasWisdom => Blessing {
            name: blessing,
            purchasing_cost: 500.0,
            description: "The Wit!",
            display_name: "Athenas Wisdom",
            required_tier: 0,
        },
    }
}

pub fn should_unlock_blessing(input_blessing: BlessingTypes, game: &Game) -> bool {
    let blessing = &game.world.blessings[input_blessing as usize];
    if blessing.required_tier > game.state.rebirth_stats.class_tier {
        return false;
    }
    if blessing.purchasing_cost > game.state.items.divine_favor {
        return false;
    }
    true
}

pub fn should_be_visible_blessing(input_blessing: BlessingTypes, game: &Game) -> bool {
    let blessing = &game.world.blessings[input_blessing as usize];
    blessing.required_tier <= game.state.rebirth_stats.class_tier
}

pub fn get_blessings() -> [Blessing; BLESSING_SIZE] {
    let mut blessings: [MaybeUninit<Blessing>; BLESSING_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in BlessingTypes::iter() {
        blessings[name as usize].write(translate_blessing(name));
    }
    unsafe { mem::transmute(blessings) }
}
