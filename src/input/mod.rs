pub mod work;

use work::Work;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub work: Work,
}

impl Input {
    pub fn new() -> Input {
        Input { work: Work::Mines }
    }
}
