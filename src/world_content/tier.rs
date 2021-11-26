use crate::input::stat::STAT_SIZE;
use crate::state::stats::{get_stats_base, BaseStats, Stat};
use serde::Serialize;

#[derive(Serialize)]
pub struct Tier {
    pub level: u32,
    pub title: &'static str,
    pub description: &'static str,
    pub unlocks: Vec<&'static str>,
    pub starting_stats: [Stat; STAT_SIZE],
    pub starting_health: f64,
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
            title: "Orphant slave",
            description: "The unluckies souls in this harsh world.",
            unlocks: vec!["Only the basics of life itself is available."],
            starting_stats: get_stats_base(BaseStats {
                str: 5.0,
                int: 5.0,
                cha: 5.0,
                con: 5.0,
                dex: 5.0,
                faith: 5.0,
            }),
            starting_health: -0.3,
            purchasing_cost: 0.0,
        },
        Tier {
            level: 1,
            title: "Slave",
            description: "That there are those who have it worse, but it is of little comfort in your current life."
                ,
            unlocks: vec![
                "You can rise higher, but you are still very limited."
                    ,
            ],
            starting_stats: get_stats_base(BaseStats {
                str: 6.0,
                int: 6.0,
                cha: 6.0,
                con: 6.0,
                dex: 6.0,
                faith: 6.0,
            }),
            starting_health: -0.2,
            purchasing_cost: 2.0,
        },
        Tier {
            level: 2,
            title: "Farmless Peasant",
            description: "You own your own life, but nothing else."
                ,
            unlocks: vec![
                "The farming job."
                    ,
                "Military Service"
                    ,
            ],
            starting_stats: get_stats_base(BaseStats {
                str: 7.0,
                int: 7.0,
                cha: 7.0,
                con: 7.0,
                dex: 7.0,
                faith: 7.0,
            }),
            starting_health: 0.0,
            purchasing_cost: 4.0,
        },
        Tier {
            level: 3,
            title: "Estate Peasant",
            description: "You own your own piece of heaven"
                ,
            unlocks: vec![
                "Crafting of higher tier goods"
                    ,
                "Honorable military service is no longer out of your reach"
                    ,
            ],
            starting_stats: get_stats_base(BaseStats {
                str: 8.0,
                int: 8.0,
                cha: 8.0,
                con: 8.0,
                dex: 8.0,
                faith: 8.0,
            }),
            starting_health: 0.1,
            purchasing_cost: 8.0,
        },
    ];
}
