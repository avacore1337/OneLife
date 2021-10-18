use crate::input::work::Work as InputWork;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone)]
pub struct RebirthStats {
    pub rebirth_count: u32,
    pub class_tier: u32,
    pub coins: f64,
    pub karma: f64,
    pub time_factor: f64,
    pub max_job_levels: Vec<u32>,
}

impl RebirthStats {
    pub fn new() -> RebirthStats {
        RebirthStats {
            rebirth_count: 0,
            class_tier: 0,
            coins: 0.0,
            karma: 0.0,
            time_factor: 1.0,
            max_job_levels: new_works(),
        }
    }
}

pub fn new_works() -> Vec<u32> {
    let mut works = Vec::<u32>::new();
    for _input_work in InputWork::iter() {
        works.push(0);
    }
    works
}
