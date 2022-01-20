use crate::engine::intermediate_state::{Gain, IntermediateState};
use crate::engine::value_keys::KeyValues;
use crate::game::Game;
// use crate::input::stat::StatTypes;
use crate::input::work::{WorkCategoryTypes, WorkTypes, WORK_SIZE};
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
    pub work_type: WorkCategoryTypes,
    pub xp_req_modifier: f64,
}

impl Gain for Work {
    fn gain(&self, intermediate: &mut IntermediateState) {
        match self.name {
            WorkTypes::Mines => {
                intermediate.add_base(KeyValues::Health, -5.0, self.display_name);
            }
            WorkTypes::Latrine => {
                intermediate.add_base(KeyValues::Health, -4.0, self.display_name);
            }
            WorkTypes::GalleyRower => {
                intermediate.add_base(KeyValues::Health, -3.0, self.display_name);
            }
            WorkTypes::Fields => {
                intermediate.add_base(KeyValues::Health, -2.0, self.display_name);
            }
            WorkTypes::Mill => {
                intermediate.add_base(KeyValues::Health, -1.0, self.display_name);
            }
            WorkTypes::Priest => {
                intermediate.add_base(KeyValues::DivineFavor, 1.0, self.display_name);
            }
            WorkTypes::Bishop => {
                intermediate.add_base(KeyValues::DivineFavor, 1.5, self.display_name);
            }

            _ => (),
        }
    }
}

const fn translate_work(work: WorkTypes) -> Work {
    match work {
        WorkTypes::Mines => Work {
            name: work,
            money: 0.5,
            description: "Hard labor that kills you",
            display_name: "The Mines",
            required_tier: 0,
            work_type: WorkCategoryTypes::Labor,
            xp_req_modifier: 1.0,
        },
        WorkTypes::Latrine => Work {
            name: work,
            money: 1.0,
            description: "A shitty job",
            display_name: "Latrine Duty",
            required_tier: 0,
            work_type: WorkCategoryTypes::Labor,
            xp_req_modifier: 2.0,
        },
        WorkTypes::GalleyRower => Work {
            name: work,
            money: 2.5,
            description: "Row row row your boat",
            display_name: "Gallery Rower",
            required_tier: 0,
            work_type: WorkCategoryTypes::Labor,
            xp_req_modifier: 4.0,
        },
        WorkTypes::Fields => Work {
            name: work,
            money: 5.0,
            description: "You ain't picking flowers",
            display_name: "Field Work",
            required_tier: 0,
            work_type: WorkCategoryTypes::Labor,
            xp_req_modifier: 8.0,
        },
        WorkTypes::Mill => Work {
            name: work,
            money: 8.0,
            description: "Hard labor",
            display_name: "Mill Worker",
            required_tier: 0,
            work_type: WorkCategoryTypes::Labor,
            xp_req_modifier: 16.0,
        },
        WorkTypes::Weaver => Work {
            name: work,
            money: 14.0,
            description: "Real work",
            display_name: "Weaver",
            required_tier: 1,
            work_type: WorkCategoryTypes::Labor,
            xp_req_modifier: 32.0,
        },
        WorkTypes::Fisherman => Work {
            name: work,
            money: 17.0,
            description: "A man of the sea",
            display_name: "Fisherman",
            required_tier: 2,
            work_type: WorkCategoryTypes::Labor,
            xp_req_modifier: 64.0,
        },
        WorkTypes::Farmer => Work {
            name: work,
            money: 32.0,
            description: "Hard labor for a free man",
            display_name: "Farmer",
            required_tier: 3,
            work_type: WorkCategoryTypes::Labor,
            xp_req_modifier: 128.0,
        },
        WorkTypes::Woodcutter => Work {
            name: work,
            money: 128.0,
            description: "Hard labor for a free man",
            display_name: "Woodcutter",
            required_tier: 4,
            work_type: WorkCategoryTypes::Labor,
            xp_req_modifier: 2_000.0,
        },
        WorkTypes::LeatherWorker => Work {
            name: work,
            money: 70.0,
            description: "Hard labor for a free man",
            display_name: "LeatherWorker",
            required_tier: 5,
            work_type: WorkCategoryTypes::Labor,
            xp_req_modifier: 512.0,
        },
        WorkTypes::Potter => Work {
            name: work,
            money: 120.0,
            description: "Hard labor for a free man",
            display_name: "Potter",
            required_tier: 6,
            work_type: WorkCategoryTypes::Labor,
            xp_req_modifier: 2.0e3,
        },
        WorkTypes::Carpenter => Work {
            name: work,
            money: 240.0,
            description: "Hard labor for a free man",
            display_name: "Carpenter",
            required_tier: 7,
            work_type: WorkCategoryTypes::Labor,
            xp_req_modifier: 8.0e3,
        },
        WorkTypes::BlacksmithApprentice => Work {
            name: work,
            money: 400.0,
            description: "Hope to become muscular",
            display_name: "Blacksmith Apprentice",
            required_tier: 7,
            work_type: WorkCategoryTypes::Labor,
            xp_req_modifier: 32.0e3,
        },
        WorkTypes::Blacksmith => Work {
            name: work,
            money: 90.0,
            description: "Hammer time",
            display_name: "Blacksmith",
            required_tier: 8,
            work_type: WorkCategoryTypes::Labor,
            xp_req_modifier: 2048.0,
        },
        WorkTypes::Goldsmith => Work {
            name: work,
            money: 90.0,
            description: "Hammer time",
            display_name: "Goldsmith",
            required_tier: 8,
            work_type: WorkCategoryTypes::Labor,
            xp_req_modifier: 2048.0,
        },
        WorkTypes::GlassBlower => Work {
            name: work,
            money: 90.0,
            description: "Hammer time",
            display_name: "Glassblower",
            required_tier: 8,
            work_type: WorkCategoryTypes::Labor,
            xp_req_modifier: 2048.0,
        },
        WorkTypes::Weaponsmith => Work {
            name: work,
            money: 90.0,
            description: "Hammer time",
            display_name: "Weaponsmith",
            required_tier: 9,
            work_type: WorkCategoryTypes::Labor,
            xp_req_modifier: 2048.0,
        },
        // ------------------Soldiers ---------------------
        WorkTypes::BagageBoy => Work {
            name: work,
            money: 10.0,
            description: "todo",
            display_name: "Bagage Boy",
            required_tier: 2,
            work_type: WorkCategoryTypes::Soldier,
            xp_req_modifier: 64.0,
        },
        WorkTypes::Slinger => Work {
            name: work,
            money: 15.0,
            description: "todo",
            display_name: "Slinger",
            required_tier: 2,
            work_type: WorkCategoryTypes::Soldier,
            xp_req_modifier: 128.0,
        },
        WorkTypes::Peltasts => Work {
            name: work,
            money: 30.0,
            description: "todo",
            display_name: "Peltasts",
            required_tier: 2,
            work_type: WorkCategoryTypes::Soldier,
            xp_req_modifier: 256.0,
        },
        WorkTypes::Pikeman => Work {
            name: work,
            money: 60.0,
            description: "todo",
            display_name: "Pikeman",
            required_tier: 2,
            work_type: WorkCategoryTypes::Soldier,
            xp_req_modifier: 512.0,
        },
        WorkTypes::FootCompanion => Work {
            name: work,
            money: 120.0,
            description: "todo",
            display_name: "FootCompanion",
            required_tier: 2,
            work_type: WorkCategoryTypes::Soldier,
            xp_req_modifier: 1000.0,
        },
        WorkTypes::Hypaspists => Work {
            name: work,
            money: 360.0,
            description: "todo",
            display_name: "Hypaspists",
            required_tier: 3,
            work_type: WorkCategoryTypes::Soldier,
            xp_req_modifier: 4_000.0,
        },
        WorkTypes::LightCavalery => Work {
            name: work,
            money: 800.0,
            description: "todo",
            display_name: "Light Cavalery",
            required_tier: 4,
            work_type: WorkCategoryTypes::Soldier,
            xp_req_modifier: 32_000.0,
        },
        WorkTypes::Priest => Work {
            name: work,
            money: 800.0,
            description: "todo",
            display_name: "Priest",
            required_tier: 4,
            work_type: WorkCategoryTypes::Priest,
            xp_req_modifier: 1_000.0,
        },
        WorkTypes::Bishop => Work {
            name: work,
            money: 800.0,
            description: "todo",
            display_name: "Bishop",
            required_tier: 4,
            work_type: WorkCategoryTypes::Priest,
            xp_req_modifier: 4_000.0,
        },
        // Int based work
        // WorkTypes::Trader => Work {
        //     name: work,
        //     money: 150.0,
        //     description: "Sell low, Buy when high",
        //     display_name: "Trader",
        //     required_tier: 7,
        //     work_type: StatTypes::Intellectual,
        //     xp_req_modifier: 4_000.0,
        // },
    }
}

pub fn should_unlock_work(work_type: WorkTypes, game: &Game) -> bool {
    let work = game.world.get_work(work_type);
    if work.required_tier > game.state.rebirth_stats.tier {
        return false;
    }
    match work_type {
        WorkTypes::Mines => true,
        WorkTypes::Latrine => {
            game.state.works[work_type as usize - 1].level >= 10
                || game.state.rebirth_stats.tier >= 1
        }
        WorkTypes::GalleyRower => {
            game.state.works[work_type as usize - 1].level >= 10
                || game.state.rebirth_stats.tier >= 2
        }
        WorkTypes::Fields => {
            game.state.works[work_type as usize - 1].level >= 10
                || game.state.rebirth_stats.tier >= 3
        }
        WorkTypes::Mill => {
            game.state.works[work_type as usize - 1].level >= 10
                || game.state.rebirth_stats.tier >= 4
        }
        WorkTypes::Weaver => {
            game.state.works[work_type as usize - 1].level >= 10
                || game.state.rebirth_stats.tier >= 5
        }
        WorkTypes::Fisherman => {
            game.state.works[work_type as usize - 1].level >= 10
                || game.state.rebirth_stats.tier >= 6
        }
        WorkTypes::Farmer => game.state.works[work_type as usize - 1].level >= 10,
        WorkTypes::Woodcutter => game.state.works[work_type as usize - 1].level >= 10,
        WorkTypes::LeatherWorker => game.state.works[work_type as usize - 1].level >= 10,
        WorkTypes::Potter => game.state.works[work_type as usize - 1].level >= 10,
        WorkTypes::Carpenter => game.state.works[work_type as usize - 1].level >= 10,
        WorkTypes::BlacksmithApprentice => game.state.works[work_type as usize - 1].level >= 10,
        WorkTypes::Blacksmith => game.state.works[work_type as usize - 1].level >= 10,
        WorkTypes::Goldsmith => game.state.works[work_type as usize - 1].level >= 10,
        WorkTypes::GlassBlower => game.state.works[work_type as usize - 1].level >= 10,
        WorkTypes::Weaponsmith => game.state.works[work_type as usize - 1].level >= 10,
        // WorkTypes::Trader => game.state.works[work_type as usize - 1].level >= 10,

        // Soldier
        WorkTypes::BagageBoy => true,
        WorkTypes::Slinger => game.state.works[work_type as usize - 1].level >= 10,
        WorkTypes::Peltasts => game.state.works[work_type as usize - 1].level >= 10,
        WorkTypes::Pikeman => game.state.works[work_type as usize - 1].level >= 10,
        WorkTypes::FootCompanion => game.state.works[work_type as usize - 1].level >= 10,
        WorkTypes::Hypaspists => game.state.works[work_type as usize - 1].level >= 10,
        WorkTypes::LightCavalery => game.state.works[work_type as usize - 1].level >= 10,

        //Priest/Faith
        WorkTypes::Priest => true,
        WorkTypes::Bishop => game.state.works[work_type as usize - 1].level >= 10,
    }
}

pub fn should_be_visible_work(input_work: WorkTypes, game: &Game) -> bool {
    let work = game.world.get_work(input_work);
    work.required_tier <= game.state.rebirth_stats.tier
}

pub fn get_works() -> [Work; WORK_SIZE] {
    let mut works: [MaybeUninit<Work>; WORK_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in WorkTypes::iter() {
        works[name as usize].write(translate_work(name));
    }
    unsafe { mem::transmute(works) }
}
