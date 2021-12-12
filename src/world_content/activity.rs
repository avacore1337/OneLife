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
    pub display_name: &'static str,
    pub required_tier: u32,
}

impl Gain for Activity {
    fn gain(&self, intermediate: &mut IntermediateState) {
        match self.name {
            ActivityTypes::Training => {
                intermediate.set_base(KeyValues::Str, 10.0);
            }
            ActivityTypes::Studying => {
                intermediate.set_base(KeyValues::Int, 10.0);
            }
            ActivityTypes::Flirt => {
                intermediate.set_base(KeyValues::Cha, 10.0);
            }
            ActivityTypes::Run => {
                intermediate.set_base(KeyValues::Con, 10.0);
            }
            ActivityTypes::Acrobatics => {
                intermediate.set_base(KeyValues::Dex, 10.0);
            }
            ActivityTypes::Praying => {
                intermediate.set_base(KeyValues::Faith, 10.0);
            }
            ActivityTypes::Meditate => {
                intermediate.set_base(KeyValues::Mindfull, 10.0);
            }
            ActivityTypes::WarGames => {
                intermediate.set_base(KeyValues::Tactics, 10.0);
            }
        }
    }
}

pub fn translate_activity(activity: ActivityTypes) -> Activity {
    match activity {
        ActivityTypes::Training => Activity {
            name: activity,
            description: "Getting stronger, faster",
            display_name: "Weight Lifting",
            required_tier: 2,
        },
        ActivityTypes::Studying => Activity {
            name: activity,
            description: "Getting smarter",
            display_name: "Studying",
            required_tier: 0,
        },
        ActivityTypes::Flirt => Activity {
            name: activity,
            description: "Getting what you want",
            display_name: "Flirt",
            required_tier: 1,
        },
        ActivityTypes::Run => Activity {
            name: activity,
            description: "Going further",
            display_name: "Running",
            required_tier: 0,
        },
        ActivityTypes::Acrobatics => Activity {
            name: activity,
            description: "Getting bendier",
            display_name: "Acrobatics",
            required_tier: 7,
        },
        ActivityTypes::Praying => Activity {
            name: activity,
            description: "Getting more pieus",
            display_name: "Praying",
            required_tier: 5,
        },
        ActivityTypes::Meditate => Activity {
            name: activity,
            description: "Introspect on your being",
            display_name: "Meditate",
            required_tier: 2,
        },
        ActivityTypes::WarGames => Activity {
            name: activity,
            description: "Train tactics",
            display_name: "War Games",
            required_tier: 2,
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
    //     ActivityTypes::Fields => game.state.activitys[ActivityTypes::Mines as usize].level > 10,
    // }
}

pub fn should_be_visible_activity(input_activity: ActivityTypes, game: &Game) -> bool {
    let activity = &game.world.activities[input_activity as usize];
    activity.required_tier <= game.state.rebirth_stats.class_tier
}

pub fn get_activities() -> [Activity; ACTIVITY_SIZE] {
    let mut activitys: [MaybeUninit<Activity>; ACTIVITY_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in ActivityTypes::iter() {
        activitys[name as usize].write(translate_activity(name));
    }
    unsafe { mem::transmute(activitys) }
}
