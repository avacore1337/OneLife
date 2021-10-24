use crate::input::work::Work as InputWork;
use serde::Serialize;
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub struct Work {
    pub name: InputWork,
    pub money: f64,
    pub description: &'static str,
    pub required_tier: u32,
    // pub stat_gains: BaseStats,
}

pub fn translate_work(work: InputWork) -> Work {
    match work {
        InputWork::Mines => Work {
            name: InputWork::Mines,
            money: 0.5,
            description: "Hard labor that kills you",
            required_tier: 0,
        },
        InputWork::Fields => Work {
            name: InputWork::Fields,
            money: 1.0,
            description: "Hard labor that kills you slowly",
            required_tier: 0,
        },
        InputWork::Servant => Work {
            name: InputWork::Servant,
            money: 2.0,
            description: "Hard labor",
            required_tier: 0,
        },
        InputWork::Teacher => Work {
            name: InputWork::Teacher,
            money: 3.0,
            description: "Good labor ",
            required_tier: 1,
        },
        InputWork::Farm => Work {
            name: InputWork::Farm,
            money: 5.0,
            description: "Hard labor for a free man",
            required_tier: 2,
        },
    }
}

pub fn get_works() -> Vec<Work> {
    let mut works = Vec::<Work>::new();
    for input_work in InputWork::iter() {
        works.push(translate_work(input_work));
    }
    works
}