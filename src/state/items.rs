use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Items {
    pub coins: f64,
}

impl Items {
    pub fn new() -> Items {
        Items { coins: 0.0 }
    }
}
