use crate::state::stats::BaseStats;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tier {
    pub title: String,
    pub desrcription: String,
    pub unlocks: Vec<String>,
    pub starting_stats: BaseStats,
}

// T1 Orphant slave
// T2 Slave
// T3 Farmless Peasant
// T4 Estate peasant
// T5 Landowning peasant
// T6 Farm owner
// T7 Estate owner
// T8 Grand estate owner
// T9 low Noble
// T10 middle Noble
// T11 high Noble
// T12 Regent
// T13 King
// T14 King of Kings
// T15 The August one.
pub fn init_tiers() -> Vec<Tier> {
    return vec![
        Tier {
            title: "Orphant slave".to_string(),
            desrcription: "The unluckies souls in this harsh world.".to_string(),
            unlocks: vec!["Only the basics of life itself is available.".to_string()],
            starting_stats: BaseStats {
                str: 5.0,
                int: 5.0,
                cha: 5.0,
                con: 5.0,
                dex: 5.0,
                faith: 5.0,
            }
        },
        Tier {
            title: "Slave".to_string(),
            desrcription: "That there are those who have it worse, but it is of little comfort in your current life."
                .to_string(),
            unlocks: vec![
                "You can rise higher, but you are still very limited."
                    .to_string(),
            ],
            starting_stats: BaseStats {
                str: 6.0,
                int: 6.0,
                cha: 6.0,
                con: 6.0,
                dex: 6.0,
                faith: 6.0,
            }
        },
    ];
}
