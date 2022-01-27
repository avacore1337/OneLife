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
    pub effect_description: &'static str,
    pub icon: &'static str,
    pub required_tier: u32,
}

impl Gain for Activity {
    fn gain(&self, intermediate: &mut IntermediateState) {
        match self.name {
            ActivityTypes::Training => {
                intermediate.set_base(KeyValues::Str, 10.0, self.display_name);
            }
            ActivityTypes::Studying => {
                intermediate.set_base(KeyValues::Int, 10.0, self.display_name);
            }
            ActivityTypes::Flirt => {
                intermediate.set_base(KeyValues::Cha, 10.0, self.display_name);
            }
            ActivityTypes::Run => {
                intermediate.set_base(KeyValues::Con, 10.0, self.display_name);
            }
            ActivityTypes::Acrobatics => {
                intermediate.set_base(KeyValues::Dex, 10.0, self.display_name);
            }
            ActivityTypes::Praying => {
                intermediate.set_base(KeyValues::Faith, 10.0, self.display_name);
            }
            ActivityTypes::Meditate => {
                intermediate.set_base(KeyValues::Mindfull, 10.0, self.display_name);
            }
            ActivityTypes::WarGames => {
                intermediate.set_base(KeyValues::Tactics, 10.0, self.display_name);
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
            effect_description: "10 Strength XP/s",
            icon: "dumbbell",
            required_tier: 2,
        },
        ActivityTypes::Studying => Activity {
            name: activity,
            description: "Getting smarter",
            display_name: "Studying",
            effect_description: "10 Intelligence XP/s",
            icon: "brain",
            required_tier: 0,
        },
        ActivityTypes::Flirt => Activity {
            name: activity,
            description: "Getting what you want",
            display_name: "Flirt",
            effect_description: "10 Charisma XP/s",
            icon: "glass-cheers",
            required_tier: 1,
        },
        ActivityTypes::Run => Activity {
            name: activity,
            description: "Going further",
            display_name: "Running",
            effect_description: "10 Constitution XP/s",
            icon: "running",
            required_tier: 0,
        },
        ActivityTypes::Acrobatics => Activity {
            name: activity,
            description: "Getting bendier",
            display_name: "Acrobatics",
            effect_description: "10 Dexterity XP/s",
            icon: "dumbbell",
            required_tier: 7,
        },
        ActivityTypes::Praying => Activity {
            name: activity,
            description: "Getting more pieus",
            display_name: "Praying",
            effect_description: "10 Piety XP/s",
            icon: "pray",
            required_tier: 3,
        },
        ActivityTypes::Meditate => Activity {
            name: activity,
            description: "Introspect on your being",
            display_name: "Meditate",
            effect_description: "10 Mindfullness XP/s",
            icon: "hand-holding",
            required_tier: 3,
        },
        ActivityTypes::WarGames => Activity {
            name: activity,
            description: "Train tactics",
            display_name: "War Games",
            effect_description: "10 Military Tactics XP/s",
            icon: "map",
            required_tier: 3,
        },
    }
}

pub fn should_unlock_activity(input_activity: ActivityTypes, game: &Game) -> bool {
    let activity = game.world.get_activity(input_activity);
    if activity.required_tier > game.state.rebirth_stats.tier {
        return false;
    }
    true
}

pub fn should_be_visible_activity(input_activity: ActivityTypes, game: &Game) -> bool {
    let activity = game.world.get_activity(input_activity);
    if activity.required_tier > game.state.rebirth_stats.tier {
        return false;
    }
    match input_activity {
        ActivityTypes::Meditate => game.state.rebirth_stats.unlocks.has_meditation,
        ActivityTypes::WarGames => game.state.rebirth_stats.unlocks.has_military_tactics,
        ActivityTypes::Praying => game.state.rebirth_stats.unlocks.has_faith,
        _ => true,
    }
}

pub fn get_activities() -> [Activity; ACTIVITY_SIZE] {
    let mut activitys: [MaybeUninit<Activity>; ACTIVITY_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in ActivityTypes::iter() {
        activitys[name as usize].write(translate_activity(name));
    }
    unsafe { mem::transmute(activitys) }
}
