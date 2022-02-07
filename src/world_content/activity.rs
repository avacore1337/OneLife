use crate::engine::intermediate_state::{Gain, IntermediateState};
use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::icon::{Icon, IconType};
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
    pub gained_value_type: KeyValues,
    pub base_gain_amount: f64,
    pub icon: Icon,
    pub required_tier: u32,
}

impl Gain for Activity {
    fn gain(&self, intermediate: &mut IntermediateState) {
        intermediate.set_base(
            self.gained_value_type,
            self.base_gain_amount,
            self.display_name,
        );
    }
}

pub fn translate_activity(activity: ActivityTypes) -> Activity {
    match activity {
        ActivityTypes::Training => Activity {
            name: activity,
            description: "Getting stronger, faster",
            display_name: "Weight Lifting",
            effect_description: "Strength",
            gained_value_type: KeyValues::Str,
            base_gain_amount: 10.0,
            icon: IconType::Str.into(),
            required_tier: 2,
        },
        ActivityTypes::Studying => Activity {
            name: activity,
            description: "Getting smarter",
            display_name: "Studying",
            effect_description: "Intelligence",
            gained_value_type: KeyValues::Int,
            base_gain_amount: 10.0,
            icon: IconType::Int.into(),
            required_tier: 0,
        },
        ActivityTypes::Flirt => Activity {
            name: activity,
            description: "Getting what you want",
            display_name: "Flirt",
            effect_description: "Charisma",
            gained_value_type: KeyValues::Cha,
            base_gain_amount: 10.0,
            icon: IconType::Cha.into(),
            required_tier: 1,
        },
        ActivityTypes::Run => Activity {
            name: activity,
            description: "Going further",
            display_name: "Running",
            effect_description: "Constitution",
            gained_value_type: KeyValues::Con,
            base_gain_amount: 10.0,
            icon: IconType::Con.into(),
            required_tier: 0,
        },
        ActivityTypes::Acrobatics => Activity {
            name: activity,
            description: "Getting bendier",
            display_name: "Acrobatics",
            effect_description: "Dexterity",
            gained_value_type: KeyValues::Dex,
            base_gain_amount: 10.0,
            icon: IconType::Dex.into(),
            required_tier: 7,
        },
        ActivityTypes::Praying => Activity {
            name: activity,
            description: "Getting more pieus",
            display_name: "Praying",
            effect_description: "Piety",
            gained_value_type: KeyValues::Faith,
            base_gain_amount: 10.0,
            icon: IconType::Faith.into(),
            required_tier: 3,
        },
        ActivityTypes::Meditate => Activity {
            name: activity,
            description: "Introspect on your being",
            display_name: "Meditate",
            effect_description: "Mindfullness",
            gained_value_type: KeyValues::Mindfull,
            base_gain_amount: 10.0,
            icon: IconType::Mindfull.into(),
            required_tier: 3,
        },
        ActivityTypes::WarGames => Activity {
            name: activity,
            description: "Train tactics",
            display_name: "War Games",
            effect_description: "Military Tactics",
            gained_value_type: KeyValues::Tactics,
            base_gain_amount: 10.0,
            icon: IconType::Tactics.into(),
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
