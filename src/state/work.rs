use crate::input::work::Work as InputWork;
use serde::{Deserialize, Serialize};
use std::mem::size_of;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Work {
    pub name: InputWork,
    pub current_level: u32,
    pub next_level_progress: f64,
    pub next_level_percentage: f64,
    //pub max_level: u32,
}

impl Work {
    pub fn new(work: InputWork) -> Work {
        Work {
            name: work,
            current_level: 0,
            next_level_progress: 0.0,
            next_level_percentage: 0.0,
            //max_level: 0,
        }
    }
}

pub fn get_works() -> [Work; size_of::<InputWork>()] {
    let mut works: [MaybeUninit<Work>; size_of::<InputWork>()] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in InputWork::iter() {
        works[name as usize].write(Work::new(name));
    }
    unsafe { mem::transmute(works) }
}
