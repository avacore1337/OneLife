use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct MetaData {
    pub game_speed: u32,
}
impl MetaData {
    pub fn new() -> MetaData {
        MetaData { game_speed: 10 }
    }
}
