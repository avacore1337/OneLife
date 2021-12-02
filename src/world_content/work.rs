use crate::engine::intermediate_state::{Gain, IntermediateState};
use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::input::stat::StatTypes;
use crate::input::work::{WorkTypes, WORK_SIZE};
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub struct Work {
    pub name: WorkTypes,
    pub money: f64,
    pub description: &'static str,
    pub display_name: &'static str,
    pub required_tier: u32,
    pub main_stat: StatTypes,
}

impl Gain for Work {
    fn gain(&self, intermediate: &mut IntermediateState) {
        match self.name {
            WorkTypes::Mines => {
                intermediate.set_base(KeyValues::Health, -5.0);
            }
            WorkTypes::Latrine => {
                intermediate.set_base(KeyValues::Health, -4.0);
            }
            WorkTypes::GalleyRower => {
                intermediate.set_base(KeyValues::Health, -3.0);
            }
            WorkTypes::Fields => {
                intermediate.set_base(KeyValues::Health, -2.0);
            }
            WorkTypes::Mill => {
                intermediate.set_base(KeyValues::Health, -1.0);
            }
            _ => (),
        }
    }
}

pub const fn translate_work(work: WorkTypes) -> Work {
    match work {
        WorkTypes::Mines => Work {
            name: work,
            money: 0.5,
            description: "Hard labor that kills you",
            display_name: "The Mines",
            required_tier: 0,
            main_stat: StatTypes::Str,
        },
        WorkTypes::Latrine => Work {
            name: work,
            money: 1.0,
            description: "A shitty job",
            display_name: "Latrine Duty",
            required_tier: 0,
            main_stat: StatTypes::Str,
        },
        WorkTypes::GalleyRower => Work {
            name: work,
            money: 2.5,
            description: "Row row row your boat",
            display_name: "Gallery Rower",
            required_tier: 0,
            main_stat: StatTypes::Str,
        },
        WorkTypes::Fields => Work {
            name: work,
            money: 5.0,
            description: "You ain't picking flowers",
            display_name: "Field Work",
            required_tier: 0,
            main_stat: StatTypes::Str,
        },
        WorkTypes::Mill => Work {
            name: work,
            money: 9.0,
            description: "Hard labor",
            display_name: "Mill Worker",
            required_tier: 0,
            main_stat: StatTypes::Str,
        },
        WorkTypes::Weaver => Work {
            name: work,
            money: 14.0,
            description: "Real work",
            display_name: "Weaver",
            required_tier: 1,
            main_stat: StatTypes::Str,
        },
        WorkTypes::Fisherman => Work {
            name: work,
            money: 20.0,
            description: "A man of the sea",
            display_name: "Fisherman",
            required_tier: 1,
            main_stat: StatTypes::Str,
        },
        WorkTypes::Farmer => Work {
            name: work,
            money: 40.0,
            description: "Hard labor for a free man",
            display_name: "Farmer",
            required_tier: 2,
            main_stat: StatTypes::Str,
        },
        // WorkTypes::Servant => Work {
        //     name: work,
        //     money: 2.0,
        //     description: "Hard labor",
        //     display_name: "Mines",
        //     required_tier: 0,
        // },
        // WorkTypes::Teacher => Work {
        //     name: work,
        //     money: 3.0,
        //     description: "Good labor ",
        //     display_name: "Mines",
        //     required_tier: 1,
        // },
    }
}

pub fn should_unlock_work(input_work: WorkTypes, game: &Game) -> bool {
    let work = &game.world.works[input_work as usize];
    if work.required_tier > game.state.rebirth_stats.class_tier {
        return false;
    }
    match input_work {
        WorkTypes::Mines => true,
        WorkTypes::Latrine => game.state.works[WorkTypes::Mines as usize].level >= 10,
        WorkTypes::GalleyRower => game.state.works[WorkTypes::Latrine as usize].level >= 10,
        WorkTypes::Fields => game.state.works[WorkTypes::GalleyRower as usize].level >= 10,
        WorkTypes::Mill => game.state.works[WorkTypes::Fields as usize].level >= 10,
        WorkTypes::Weaver => game.state.works[WorkTypes::Mill as usize].level >= 10,
        WorkTypes::Fisherman => game.state.works[WorkTypes::Weaver as usize].level >= 10,
        WorkTypes::Farmer => game.state.works[WorkTypes::Fisherman as usize].level >= 10,
    }
}

pub fn should_be_visible_work(input_work: WorkTypes, game: &Game) -> bool {
    let work = &game.world.works[input_work as usize];
    work.required_tier <= game.state.rebirth_stats.class_tier
}

pub fn get_works() -> [Work; WORK_SIZE] {
    let mut works: [MaybeUninit<Work>; WORK_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in WorkTypes::iter() {
        works[name as usize].write(translate_work(name));
    }
    unsafe { mem::transmute(works) }
}
