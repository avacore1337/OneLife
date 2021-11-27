pub mod activity;
pub mod boost_item;
pub mod housing;
pub mod stat;
pub mod tomb;
pub mod work;

use activity::ActivityTypes;
use housing::HousingTypes;
use work::WorkTypes;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Input {
    pub work: WorkTypes,
    pub housing: HousingTypes,
    pub activity: ActivityTypes,
}

impl Input {
    pub fn new() -> Input {
        Input {
            work: WorkTypes::Mines,
            housing: HousingTypes::StoneFloor,
            activity: ActivityTypes::Training,
        }
    }
}
