use crate::input::work::Work as InputWork;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Work {
    pub current_level: u32,
    pub next_level_progress: f64,
    pub max_level: u32,
}

impl Work {
    pub fn new() -> Work {
        Work {
            current_level: 0,
            next_level_progress: 0.0,
            max_level: 0,
        }
    }
}

impl Default for Work {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_works() -> Vec<Work> {
    let mut works = Vec::<Work>::new();
    for _ in InputWork::iter() {
        works.push(Work::default());
    }
    works
}
