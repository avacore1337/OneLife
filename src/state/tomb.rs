use crate::input::tomb::{TombTypes, TOMB_SIZE};
use serde::{Deserialize, Serialize};
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tomb {
    pub name: TombTypes,
    pub is_purchased: bool,
    pub is_unlocked: bool,
    pub is_visible: bool,
    pub effective_income: f64,
}

impl Tomb {
    pub fn new(tomb: TombTypes) -> Tomb {
        Tomb {
            name: tomb,
            is_purchased: false,
            is_unlocked: false,
            is_visible: true,
            effective_income: 0.0,
        }
    }
}

pub fn get_tombs() -> [Tomb; TOMB_SIZE] {
    let mut tombs: [MaybeUninit<Tomb>; TOMB_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in TombTypes::iter() {
        tombs[name as usize].write(Tomb::new(name));
    }
    unsafe { mem::transmute(tombs) }
}
