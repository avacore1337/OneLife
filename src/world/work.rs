use crate::input::work::Work as InputWork;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize)]
pub struct Work {
    pub name: InputWork,
    pub money: f64,
    pub description: String,
    // pub stat_gains: BaseStats,
}

pub fn translate_work(work: &InputWork) -> Work {
    match work {
        InputWork::Mines => Work {
            name: InputWork::Mines,
            money: 0.1,
            description: "Hard labor that kills you".to_string(),
        },
        InputWork::Fields => Work {
            name: InputWork::Fields,
            money: 0.3,
            description: "Hard labor that kills you".to_string(),
        },
        InputWork::Servant => Work {
            name: InputWork::Servant,
            money: 1.0,
            description: "Hard labor that kills you".to_string(),
        },
        InputWork::Teacher => Work {
            name: InputWork::Teacher,
            money: 1.5,
            description: "Hard labor that kills you".to_string(),
        },
        InputWork::Farm => Work {
            name: InputWork::Farm,
            money: 2.0,
            description: "Hard labor that kills you".to_string(),
        },
    }
}

pub fn get_works() -> Vec<Work> {
    let mut works = Vec::<Work>::new();
    for input_work in InputWork::iter() {
        works.push(translate_work(&input_work));
    }
    works
}
