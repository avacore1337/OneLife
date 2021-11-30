use super::rebirth_upgrade::{get_rebirth_upgrades, RebirthUpgrade};
use crate::input::rebirth_upgrade::REBIRTH_UPGRADE_SIZE;
use crate::input::work::WORK_SIZE;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RebirthStats {
    pub rebirth_count: u32,
    pub class_tier: u32,
    pub coins: f64,
    pub karma: f64,
    pub time_factor: f64,
    pub max_job_levels: [u32; WORK_SIZE],
    pub rebirth_upgrades: [RebirthUpgrade; REBIRTH_UPGRADE_SIZE],
}

impl RebirthStats {
    pub fn new() -> RebirthStats {
        RebirthStats {
            rebirth_count: 0,
            class_tier: 0,
            coins: 0.0,
            karma: 0.0,
            time_factor: 1.0,
            max_job_levels: [0; WORK_SIZE],
            rebirth_upgrades: get_rebirth_upgrades(),
        }
    }
}
