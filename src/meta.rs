use js_sys::Date;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct MetaData {
    pub game_speed: u32,
    pub autosave: bool,
    pub last_save_time: f64,
}
impl MetaData {
    pub fn new() -> MetaData {
        MetaData {
            game_speed: 100,
            autosave: false,
            last_save_time: Date::new_0().get_time(),
        }
    }

    pub fn should_autosave(self) -> bool {
        let now = Date::new_0().get_time();
        self.autosave && now >= self.last_save_time + (60.0 * 1000.0)
    }

    pub fn set_savetime(&mut self) {
        self.last_save_time = Date::new_0().get_time();
    }
}
