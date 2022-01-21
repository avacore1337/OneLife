use crate::input::blessing::{BlessingTypes, BLESSING_SIZE};
use serde::{Deserialize, Serialize};
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blessing {
    pub name: BlessingTypes,
    pub is_unlocked: bool,
    pub is_visible: bool,
    pub level: u32,
    pub next_level_cost: f64,
}

impl Blessing {
    pub fn new(blessing: BlessingTypes) -> Blessing {
        Blessing {
            name: blessing,
            is_unlocked: false,
            is_visible: false,
            level: 0,
            next_level_cost: 0.0,
        }
    }
}

pub fn get_blessings() -> [Blessing; BLESSING_SIZE] {
    let mut blessings: [MaybeUninit<Blessing>; BLESSING_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in BlessingTypes::iter() {
        blessings[name as usize].write(Blessing::new(name));
    }
    unsafe { mem::transmute(blessings) }
}
