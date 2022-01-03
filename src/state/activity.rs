use crate::input::activity::{ActivityTypes, ACTIVITY_SIZE};
use serde::{Deserialize, Serialize};
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Activity {
    pub name: ActivityTypes,
    pub level: u32,
    pub next_level_progress: f64,
    pub next_level_required: f64,
    pub next_level_percentage: f64,
    pub is_unlocked: bool,
    pub is_visible: bool,
}

impl Activity {
    pub fn new(activity: ActivityTypes) -> Activity {
        Activity {
            name: activity,
            level: 0,
            next_level_progress: 0.0,
            next_level_required: 100.0,
            next_level_percentage: 0.0,
            is_unlocked: false,
            is_visible: false,
        }
    }
}

pub fn get_activities() -> [Activity; ACTIVITY_SIZE] {
    let mut activitys: [MaybeUninit<Activity>; ACTIVITY_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in ActivityTypes::iter() {
        activitys[name as usize].write(Activity::new(name));
    }
    unsafe { mem::transmute(activitys) }
}
