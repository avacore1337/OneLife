use crate::info::Info;
use js_sys::Date;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetaData {
    pub game_speed: u32,
    pub autosave: bool,
    pub last_save_time: f64,
    pub last_tick_time: f64,
    pub saved_ticks: f64,
    pub info: Info,
}
impl MetaData {
    pub fn new() -> MetaData {
        MetaData {
            game_speed: 100,
            autosave: false,
            last_save_time: Date::new_0().get_time(),
            last_tick_time: Date::new_0().get_time(),
            saved_ticks: 0.0,
            info: Info::new(),
        }
    }

    pub fn should_autosave(&self) -> bool {
        let now = Date::new_0().get_time();
        self.autosave && now >= self.last_save_time + (60.0 * 1000.0)
    }

    pub fn set_save_time(&mut self) {
        self.last_save_time = Date::new_0().get_time();
    }

    pub fn update_tick_time(&mut self) {
        let now = Date::new_0().get_time();
        if now >= self.last_tick_time + 500.0 {
            self.saved_ticks += ((now - self.last_tick_time) * crate::TICK_RATE) / 1000.0
        }
        self.last_tick_time = Date::new_0().get_time();
    }
    pub fn paused_tick_time(&mut self) {
        let now = Date::new_0().get_time();
        self.saved_ticks += 1.0;
        if now >= self.last_tick_time + 500.0 {
            self.saved_ticks += ((now - self.last_tick_time) * crate::TICK_RATE) / 1000.0 - 1.0
        }
        self.last_tick_time = Date::new_0().get_time();
    }
}
