use crate::input::tomb::{TombTypes, TOMB_SIZE};
use serde::{Deserialize, Serialize};
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Tomb {
    pub name: TombTypes,
    pub is_owned: bool,
    pub is_unlocked: bool,
    pub is_visible: bool,
}

impl Tomb {
    pub fn new(tomb: TombTypes) -> Tomb {
        Tomb {
            name: tomb,
            is_owned: false,
            is_unlocked: false,
            is_visible: true,
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
