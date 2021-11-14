use crate::engine::intermediate_state::{Gain, IntermediateState};
use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::input::work::{Work as InputWork, WORK_SIZE};
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub struct Work {
    pub name: InputWork,
    pub money: f64,
    pub description: &'static str,
    pub required_tier: u32,
    // pub stat_gains: BaseStats,
}

impl Gain for Work {
    fn gain(&self, intermediate: &mut IntermediateState) {
        match self.name {
            InputWork::Mines => {
                intermediate.set_base(KeyValues::Str, 1.0);
            }
            InputWork::Fields => {
                intermediate.set_base(KeyValues::Str, 2.0);
            }
            InputWork::Servant => {
                intermediate.set_base(KeyValues::Cha, 1.0);
            }
            InputWork::Teacher => {
                intermediate.set_base(KeyValues::Int, 1.0);
            }
            InputWork::Farm => {
                intermediate.set_base(KeyValues::Str, 3.0);
            }
        }
    }
}

pub fn translate_work(work: InputWork) -> Work {
    match work {
        InputWork::Mines => Work {
            name: InputWork::Mines,
            money: 0.5,
            description: "Hard labor that kills you",
            required_tier: 0,
        },
        InputWork::Fields => Work {
            name: InputWork::Fields,
            money: 1.0,
            description: "Hard labor that kills you slowly",
            required_tier: 0,
        },
        InputWork::Servant => Work {
            name: InputWork::Servant,
            money: 2.0,
            description: "Hard labor",
            required_tier: 0,
        },
        InputWork::Teacher => Work {
            name: InputWork::Teacher,
            money: 3.0,
            description: "Good labor ",
            required_tier: 1,
        },
        InputWork::Farm => Work {
            name: InputWork::Farm,
            money: 5.0,
            description: "Hard labor for a free man",
            required_tier: 2,
        },
    }
}

pub fn should_unlock_work(input_work: InputWork, game: &Game) -> bool {
    let work = &game.world.works[input_work as usize];
    if work.required_tier > game.state.rebirth_stats.class_tier {
        return false;
    }
    match input_work {
        InputWork::Mines => true,
        InputWork::Fields => game.state.works[InputWork::Mines as usize].current_level > 10,
        InputWork::Servant => game.state.works[InputWork::Fields as usize].current_level > 10,
        InputWork::Teacher => game.state.works[InputWork::Servant as usize].current_level > 10,
        InputWork::Farm => game.state.works[InputWork::Teacher as usize].current_level > 10,
    }
}

pub fn should_be_visable_work(input_work: InputWork, game: &Game) -> bool {
    let work = &game.world.works[input_work as usize];
    work.required_tier <= game.state.rebirth_stats.class_tier + 1
}

pub fn get_works() -> [Work; WORK_SIZE] {
    let mut works: [MaybeUninit<Work>; WORK_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in InputWork::iter() {
        works[name as usize].write(translate_work(name));
    }
    unsafe { mem::transmute(works) }
}
