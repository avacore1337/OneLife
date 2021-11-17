pub mod boost_item;
pub mod housing;
pub mod stat;
pub mod work;

use housing::HousingTypes;
use work::WorkTypes;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Input {
    pub work: WorkTypes,
    pub housing: HousingTypes,
}

impl Input {
    pub fn new() -> Input {
        Input {
            work: WorkTypes::Mines,
            housing: HousingTypes::StoneFloor,
        }
    }
}
