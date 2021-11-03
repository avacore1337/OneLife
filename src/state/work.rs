use crate::input::work::{Work as InputWork, WORK_SIZE};
use serde::{Deserialize, Serialize};
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

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

pub fn get_works() -> [Work; WORK_SIZE] {
    let mut works: [MaybeUninit<Work>; WORK_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in InputWork::iter() {
        works[name as usize].write(Work::new(name));
    }
    unsafe { mem::transmute(works) }
}
