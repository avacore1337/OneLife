use crate::input::work::{WorkTypes, WORK_SIZE};
use serde::{Deserialize, Serialize};
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Work {
    pub name: WorkTypes,
    pub level: u32,
    pub next_level_progress: f64,
    pub next_level_required: f64,
    pub next_level_percentage: f64,
    pub effective_income: f64,
    pub is_unlocked: bool,
    pub is_visible: bool,
}

impl Work {
    pub fn new(work: WorkTypes) -> Work {
        Work {
            name: work,
            level: 0,
            next_level_progress: 0.0,
            next_level_required: 100.0,
            next_level_percentage: 0.0,
            effective_income: 0.0,
            is_unlocked: false,
            is_visible: true,
        }
    }
}

pub fn get_works() -> [Work; WORK_SIZE] {
    let mut works: [MaybeUninit<Work>; WORK_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in WorkTypes::iter() {
        works[name as usize].write(Work::new(name));
    }
    unsafe { mem::transmute(works) }
}
