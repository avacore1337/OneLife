use super::rebirth_upgrade::{get_rebirth_upgrades, RebirthUpgrade};
use crate::input::rebirth_upgrade::REBIRTH_UPGRADE_SIZE;
use crate::input::work::WORK_SIZE;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Unlocks {
    pub can_end_early: bool,
    pub can_auto_work: bool,
    pub can_auto_living: bool,
    pub can_auto_buy_item: bool,
    pub can_auto_buy_tomb: bool,
    pub can_replay: bool,
    pub has_faith: bool,
    pub has_skills: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RebirthStats {
    pub rebirth_count: u32,
    pub tier: u32,
    pub coins: f64,
    pub karma: f64,
    pub time_factor: f64,
    pub max_job_levels: [u32; WORK_SIZE],
    pub rebirth_upgrades: [RebirthUpgrade; REBIRTH_UPGRADE_SIZE],
    pub unlocks: Unlocks,
}

impl Default for RebirthStats {
    fn default() -> RebirthStats {
        RebirthStats {
            rebirth_count: 0,
            tier: 0,
            coins: 0.0,
            karma: 0.0,
            time_factor: 1.0,
            max_job_levels: [0; WORK_SIZE],
            rebirth_upgrades: get_rebirth_upgrades(),
            unlocks: Unlocks::default(),
        }
    }
}
