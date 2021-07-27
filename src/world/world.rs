use super::tier::{init_tiers, Tier};

pub struct World {
    pub tiers: Vec<Tier>,
}

impl World {
    pub fn new() -> World {
        World {
            tiers: init_tiers(),
        }
    }
}
