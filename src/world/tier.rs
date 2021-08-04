use crate::state::stats::BaseStats;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tier {
    pub level: u32,
    pub title: String,
    pub description: String,
    pub unlocks: Vec<String>,
    pub starting_stats: BaseStats,
    pub purchasing_cost: f64,
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
            level: 0,
            title: "Orphant slave".to_string(),
            description: "The unluckies souls in this harsh world.".to_string(),
            unlocks: vec!["Only the basics of life itself is available.".to_string()],
            starting_stats: BaseStats {
                str: 5.0,
                int: 5.0,
                cha: 5.0,
                con: 5.0,
                dex: 5.0,
                faith: 5.0,
            },
            purchasing_cost: 0.0,
        },
        Tier {
            level: 1,
            title: "Slave".to_string(),
            description: "That there are those who have it worse, but it is of little comfort in your current life."
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
            },
            purchasing_cost: 2.0,
        },
        Tier {
            level: 2,
            title: "Farmless Peasant".to_string(),
            description: "You own your own life, but nothing else."
                .to_string(),
            unlocks: vec![
                "The farming job."
                    .to_string(),
                "Military Service"
                    .to_string(),
            ],
            starting_stats: BaseStats {
                str: 7.0,
                int: 7.0,
                cha: 7.0,
                con: 7.0,
                dex: 7.0,
                faith: 7.0,
            },
            purchasing_cost: 4.0,
        },
        Tier {
            level: 3,
            title: "Estate Peasant".to_string(),
            description: "You own your own piece of heaven"
                .to_string(),
            unlocks: vec![
                "Crafting of higher tier goods"
                    .to_string(),
                "Honorable military service is no longer out of your reach"
                    .to_string(),
            ],
            starting_stats: BaseStats {
                str: 8.0,
                int: 8.0,
                cha: 8.0,
                con: 8.0,
                dex: 8.0,
                faith: 8.0,
            },
            purchasing_cost: 8.0,
        },
    ];
}
