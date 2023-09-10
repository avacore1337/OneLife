use crate::input::activity::{ActivityTypes, ACTIVITY_SIZE};
use serde::{Deserialize, Serialize};
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Activity {
    pub name: ActivityTypes,
    pub is_unlocked: bool,
    pub is_visible: bool,
}

impl Activity {
    pub fn new(activity: ActivityTypes) -> Activity {
        Activity {
            name: activity,
            is_unlocked: false,
            is_visible: false,
        }
    }
}

pub fn get_activities() -> [Activity; ACTIVITY_SIZE] {
    let mut activities: [MaybeUninit<Activity>; ACTIVITY_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in ActivityTypes::iter() {
        activities[name as usize].write(Activity::new(name));
    }
    unsafe { mem::transmute(activities) }
}
