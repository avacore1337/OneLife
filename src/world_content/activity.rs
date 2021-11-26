use crate::engine::intermediate_state::{Gain, IntermediateState};
use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::input::activity::{ActivityTypes, ACTIVITY_SIZE};
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub struct Activity {
    pub name: ActivityTypes,
    pub description: &'static str,
    pub required_tier: u32,
}

impl Gain for Activity {
    fn gain(&self, intermediate: &mut IntermediateState) {
        match self.name {
            ActivityTypes::Training => {
                intermediate.set_base(KeyValues::Str, 1.0);
            }
            ActivityTypes::Studying => {
                intermediate.set_base(KeyValues::Int, 1.0);
            }
            ActivityTypes::Flirt => {
                intermediate.set_base(KeyValues::Cha, 1.0);
            }
            ActivityTypes::Run => {
                intermediate.set_base(KeyValues::Con, 1.0);
            }
            ActivityTypes::Acrobatics => {
                intermediate.set_base(KeyValues::Dex, 1.0);
            }
            ActivityTypes::Praying => {
                intermediate.set_base(KeyValues::Faith, 1.0);
            }
        }
    }
}

pub fn translate_activity(activity: ActivityTypes) -> Activity {
    match activity {
        ActivityTypes::Training => Activity {
            name: activity,
            description: "Getting stronger, faster",
            required_tier: 0,
        },
        ActivityTypes::Studying => Activity {
            name: activity,
            description: "Getting smarter",
            required_tier: 0,
        },
        ActivityTypes::Flirt => Activity {
            name: activity,
            description: "Getting what you want",
            required_tier: 1,
        },
        ActivityTypes::Run => Activity {
            name: activity,
            description: "Going further",
            required_tier: 1,
        },
        ActivityTypes::Acrobatics => Activity {
            name: activity,
            description: "Getting bendier",
            required_tier: 1,
        },
        ActivityTypes::Praying => Activity {
            name: activity,
            description: "Getting more pieus",
            required_tier: 3,
        },
    }
}

pub fn should_unlock_activity(input_activity: ActivityTypes, game: &Game) -> bool {
    let activity = &game.world.activities[input_activity as usize];
    if activity.required_tier > game.state.rebirth_stats.class_tier {
        return false;
    }
    true
    // match input_activity {
    //     ActivityTypes::Training => true,
    //     ActivityTypes::Studying => true,
    //     ActivityTypes::Fields => game.state.activitys[ActivityTypes::Mines as usize].level > 10,
    // }
}

// pub fn should_be_visable_activity(input_activity: ActivityTypes, game: &Game) -> bool {
//     let activity = &game.world.activitys[input_activity as usize];
//     activity.required_tier <= game.state.rebirth_stats.class_tier + 1
// }

pub fn get_activities() -> [Activity; ACTIVITY_SIZE] {
    let mut activitys: [MaybeUninit<Activity>; ACTIVITY_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in ActivityTypes::iter() {
        activitys[name as usize].write(translate_activity(name));
    }
    unsafe { mem::transmute(activitys) }
}
