use crate::engine::intermediate_state::{Gain, IntermediateState};
use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::input::work::{WorkTypes, WORK_SIZE};
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub struct Work {
    pub name: WorkTypes,
    pub money: f64,
    pub description: &'static str,
    pub required_tier: u32,
    // pub stat_gains: BaseStats,
}

impl Gain for Work {
    fn gain(&self, intermediate: &mut IntermediateState) {
        match self.name {
            WorkTypes::Mines => {
                intermediate.set_base(KeyValues::Str, 1.0);
            }
            WorkTypes::Fields => {
                intermediate.set_base(KeyValues::Str, 2.0);
            }
            WorkTypes::Servant => {
                intermediate.set_base(KeyValues::Cha, 1.0);
            }
            WorkTypes::Teacher => {
                intermediate.set_base(KeyValues::Int, 1.0);
            }
            WorkTypes::Farm => {
                intermediate.set_base(KeyValues::Str, 3.0);
            }
        }
    }
}

pub fn translate_work(work: WorkTypes) -> Work {
    match work {
        WorkTypes::Mines => Work {
            name: WorkTypes::Mines,
            money: 0.5,
            description: "Hard labor that kills you",
            required_tier: 0,
        },
        WorkTypes::Fields => Work {
            name: WorkTypes::Fields,
            money: 1.0,
            description: "Hard labor that kills you slowly",
            required_tier: 0,
        },
        WorkTypes::Servant => Work {
            name: WorkTypes::Servant,
            money: 2.0,
            description: "Hard labor",
            required_tier: 0,
        },
        WorkTypes::Teacher => Work {
            name: WorkTypes::Teacher,
            money: 3.0,
            description: "Good labor ",
            required_tier: 1,
        },
        WorkTypes::Farm => Work {
            name: WorkTypes::Farm,
            money: 5.0,
            description: "Hard labor for a free man",
            required_tier: 2,
        },
    }
}

pub fn should_unlock_work(input_work: WorkTypes, game: &Game) -> bool {
    let work = &game.world.works[input_work as usize];
    if work.required_tier > game.state.rebirth_stats.class_tier {
        return false;
    }
    match input_work {
        WorkTypes::Mines => true,
        WorkTypes::Fields => game.state.works[WorkTypes::Mines as usize].level > 10,
        WorkTypes::Servant => game.state.works[WorkTypes::Fields as usize].level > 10,
        WorkTypes::Teacher => game.state.works[WorkTypes::Servant as usize].level > 10,
        WorkTypes::Farm => game.state.works[WorkTypes::Teacher as usize].level > 10,
    }
}

pub fn should_be_visable_work(input_work: WorkTypes, game: &Game) -> bool {
    let work = &game.world.works[input_work as usize];
    work.required_tier <= game.state.rebirth_stats.class_tier + 1
}

pub fn get_works() -> [Work; WORK_SIZE] {
    let mut works: [MaybeUninit<Work>; WORK_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in WorkTypes::iter() {
        works[name as usize].write(translate_work(name));
    }
    unsafe { mem::transmute(works) }
}
