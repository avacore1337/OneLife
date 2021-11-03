use crate::input::work::Work as InputWork;
use serde::{Deserialize, Serialize};
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

pub fn get_works() -> Vec<Work> {
    let mut works = Vec::<Work>::new();
    for name in InputWork::iter() {
        works.push(Work::new(name));
    }
    works
}
