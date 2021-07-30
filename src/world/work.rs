use crate::input::work::Work as InputWork;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Work {
    pub money: f64,
    pub description: String,
    // pub stat_gains: BaseStats,
}

pub fn translate_work(work: &InputWork) -> Work {
    match work {
        InputWork::Mines => Work {
            money: 0.1,
            description: "Hard labor that kills you".to_string(),
        },
        InputWork::Fields => Work {
            money: 0.3,
            description: "Hard labor that kills you".to_string(),
        },
        InputWork::Servant => Work {
            money: 1.0,
            description: "Hard labor that kills you".to_string(),
        },
        InputWork::Teacher => Work {
            money: 1.5,
            description: "Hard labor that kills you".to_string(),
        },
        InputWork::Farm => Work {
            money: 2.0,
            description: "Hard labor that kills you".to_string(),
        },
    }
}
// pub fn init_tiers() -> Vec<Tier> {
//     return vec![
//         Tier {
//             title: "Orphant slave".to_string(),
//             desrcription: "The unluckies souls in this harsh world.".to_string(),
//             unlocks: vec!["Only the basics of life itself is available.".to_string()],
//             starting_stats: BaseStats {
//                 str: 5.0,
//                 int: 5.0,
//                 cha: 5.0,
//                 con: 5.0,
//                 dex: 5.0,
//                 faith: 5.0,
//             }
//         },
//         Tier {
//             title: "Slave".to_string(),
//             desrcription: "That there are those who have it worse, but it is of little comfort in your current life."
//                 .to_string(),
//             unlocks: vec![
//                 "You can rise higher, but you are still very limited."
//                     .to_string(),
//             ],
//             starting_stats: BaseStats {
//                 str: 6.0,
//                 int: 6.0,
//                 cha: 6.0,
//                 con: 6.0,
//                 dex: 6.0,
//                 faith: 6.0,
//             }
//         },
//     ];
// }
