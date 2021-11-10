use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct MetaData {
    pub game_speed: u32,
    pub autosave: bool,
    last_save_time: Duration,
}
impl MetaData {
    pub fn new() -> MetaData {
        let time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        MetaData {
            game_speed: 100,
            autosave: false,
            last_save_time: time,
        }
    }

    pub fn should_autosave(self) -> bool {
        let time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        self.autosave && time >= self.last_save_time + Duration::from_secs(60)
    }
}
