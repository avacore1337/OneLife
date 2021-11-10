pub mod boost_item;
pub mod housing;
pub mod work;

use housing::Housing;
use work::Work;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Input {
    pub work: Work,
    pub housing: Housing,
}

impl Input {
    pub fn new() -> Input {
        Input {
            work: Work::Mines,
            housing: Housing::StoneFloor,
        }
    }
}
