use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Items {
    pub money: f64,
}

impl Items {
    pub fn new() -> Items {
        Items { money: 0.0 }
    }
}
