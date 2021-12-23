use crate::input::housing::{HousingTypes, HOUSING_SIZE};
use serde::{Deserialize, Serialize};
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Housing {
    pub name: HousingTypes,
    pub is_visible: bool,
    pub is_unlocked: bool,
}

impl Housing {
    pub fn new(housing: HousingTypes) -> Housing {
        Housing {
            name: housing,
            is_visible: false,
            is_unlocked: false,
        }
    }
}

pub fn get_housings() -> [Housing; HOUSING_SIZE] {
    let mut housings: [MaybeUninit<Housing>; HOUSING_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in HousingTypes::iter() {
        housings[name as usize].write(Housing::new(name));
    }
    unsafe { mem::transmute(housings) }
}
