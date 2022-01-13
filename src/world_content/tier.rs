use crate::input::stat::STAT_SIZE;
use crate::input::work::WorkTypes;
use crate::state::stats::{get_stats_base, BaseStats, Stat};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Tier {
    pub level: u32,
    pub display_name: &'static str,
    pub description: &'static str,
    pub unlocks: Vec<&'static str>,
    pub starting_stats: [Stat; STAT_SIZE],
    pub starting_work: WorkTypes,
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
            display_name: "Orphant slave",
            description: "The unluckies souls in this harsh world.",
            unlocks: vec!["Only the basics of life itself is available."],
            starting_stats: get_stats_base(BaseStats {
                str: 0.0,
                int: 5.0,
                cha: 0.0,
                con: 5.0,
                dex: 0.0,
                faith: 0.0,
            }),
            starting_work: WorkTypes::Mines,
            starting_health: -0.3,
            purchasing_cost: 0.0,
        },
        Tier {
            level: 1,
            display_name: "Slave",
            description: "That there are those who have it worse, but it is of little comfort in your current life."
                ,
            unlocks: vec![
                "You can rise higher, but you are still very limited."
                    ,
            ],
            starting_stats: get_stats_base(BaseStats {
                str: 7.0,
                int: 7.0,
                cha: 0.0,
                con: 7.0,
                dex: 0.0,
                faith: 0.0,
            }),
            starting_work: WorkTypes::Latrine,
            starting_health: -0.2,
            purchasing_cost: 2.0,
        },
        Tier {
            level: 2,
            display_name: "Farmless Peasant",
            description: "You own your own life, but nothing else.",
            unlocks: vec![
                "The farming job."
                    ,
                "Military Service"
                    ,
            ],
            starting_stats: get_stats_base(BaseStats {
                str: 12.0,
                int: 12.0,
                cha: 12.0,
                con: 12.0,
                dex: 0.0,
                faith: 0.0,
            }),
            starting_work: WorkTypes::GalleyRower,
            starting_health: 0.0,
            purchasing_cost: 50.0,
        },
        Tier {
            level: 3,
            display_name: "Estate Peasant",
            description: "You own your own piece of heaven"
                ,
            unlocks: vec![
                "Crafting of higher tier goods",
                "Honorable military service is no longer out of your reach",
            ],
            starting_stats: get_stats_base(BaseStats {
                str: 20.0,
                int: 20.0,
                cha: 20.0,
                con: 20.0,
                dex: 0.0,
                faith: 0.0,
            }),
            starting_work: WorkTypes::Fields,
            starting_health: 0.1,
            purchasing_cost: 500.0,
        },
        Tier {
            level: 4,
            display_name: "Landowning peasant",
            description: "todo"
                ,
            unlocks: vec![ ],
            starting_stats: get_stats_base(BaseStats {
                str: 30.0,
                int: 30.0,
                cha: 30.0,
                con: 30.0,
                dex: 0.0,
                faith: 0.0,
            }),
            starting_work: WorkTypes::Mill,
            starting_health: 0.2,
            purchasing_cost: 5_000.0,
        },
        Tier {
            level: 5,
            display_name: "Farm Owner",
            description: "todo"
                ,
            unlocks: vec![ ],
            starting_stats: get_stats_base(BaseStats {
                str: 40.0,
                int: 40.0,
                cha: 40.0,
                con: 40.0,
                dex: 0.0,
                faith: 10.0,
            }),
            starting_work: WorkTypes::Weaver,
            starting_health: 0.2,
            purchasing_cost: 50_000.0,
        },
        Tier {
            level: 6,
            display_name: "Estate Owner",
            description: "todo",
            unlocks: vec![ ],
            starting_stats: get_stats_base(BaseStats {
                str: 50.0,
                int: 50.0,
                cha: 50.0,
                con: 50.0,
                dex: 0.0,
                faith: 30.0,
            }),
            starting_work: WorkTypes::Farmer,
            starting_health: 0.3,
            purchasing_cost: 500_000.0,
        },
        Tier {
            level: 7,
            display_name: "Grand Estate Owner",
            description: "todo",
            unlocks: vec![ ],
            starting_stats: get_stats_base(BaseStats {
                str: 70.0,
                int: 70.0,
                cha: 70.0,
                con: 70.0,
                dex: 0.0,
                faith: 70.0,
            }),
            starting_work: WorkTypes::Farmer,
            starting_health: 0.4,
            purchasing_cost: 5_000_000.0,
        },

    ];
}
