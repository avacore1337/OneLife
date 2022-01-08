#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Items {
    pub money: f64,
    pub income: f64,
    pub divine_favor: f64,
    pub divine_favor_rate: f64,
}

impl Items {
    pub fn new() -> Items {
        Items {
            money: 0.0,
            income: 0.0,
            divine_favor: 0.0,
            divine_favor_rate: 0.0,
        }
    }
}

impl Default for Items {
    fn default() -> Self {
        Self::new()
    }
}
