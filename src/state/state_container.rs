use super::rebirth_stats::RebirthStats;
use super::stats::BaseStats;

pub struct StateContainer {
    pub base_stats: BaseStats,
    pub rebirth_stats: RebirthStats,
}

pub fn new_game() -> StateContainer {
    StateContainer {
        base_stats: BaseStats {
            str: 10.0,
            int: 10.0,
            cha: 10.0,
            con: 10.0,
            dex: 10.0,
            faith: 10.0,
        },
        rebirth_stats: RebirthStats { class_tier: 1 },
    }
}
