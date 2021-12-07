use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub struct Options {
    pub auto_work: bool,
    pub auto_living: bool,
    pub auto_buy_item: bool,
    pub auto_buy_tomb: bool,
}
