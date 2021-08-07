use super::housing::{get_housing, Housing};
use super::tier::{init_tiers, Tier};
use super::work::{get_works, Work};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct World {
    pub tiers: Vec<Tier>,
    pub works: Vec<Work>,
    pub housing: Vec<Housing>,
}

impl World {
    pub fn new() -> World {
        World {
            tiers: init_tiers(),
            works: get_works(),
            housing: get_housing(),
        }
    }
}
