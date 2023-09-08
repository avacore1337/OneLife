use crate::info::Info;
use crate::input::options::Options;
use crate::TICK_MS;
use js_sys::Date;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetaData {
    pub game_speed: u32,
    pub autosave: bool,
    pub last_save_time: f64,
    pub last_tick_time: f64,
    pub missed_time: f64,
    pub saved_ticks: f64,
    pub info: Info,
    pub use_saved_ticks: bool,
    pub options: Options,
}

impl Default for MetaData {
    fn default() -> Self {
        Self::new()
    }
}

impl MetaData {
    pub fn new() -> MetaData {
        MetaData {
            game_speed: get_game_speed_default(),
            autosave: get_autosave_default(),
            last_save_time: Date::new_0().get_time(),
            last_tick_time: Date::new_0().get_time(),
            missed_time: 0.0,
            saved_ticks: 0.0,
            info: Info::new(),
            use_saved_ticks: false,
            options: Options::new(),
        }
    }

    pub fn should_autosave(&self) -> bool {
        let now = Date::new_0().get_time();
        self.autosave && now >= self.last_save_time + (60.0 * 1000.0)
    }

    pub fn handle_run_count(&mut self) -> u32 {
        if self.use_saved_ticks {
            if self.saved_ticks >= 5.0 {
                self.saved_ticks -= 5.0;
                5 + 1
            } else {
                let count = self.saved_ticks as u32;
                self.saved_ticks -= count as f64;
                self.use_saved_ticks = false;
                count + 1
            }
        } else {
            1
        }
    }

    pub fn set_save_time(&mut self) {
        self.last_save_time = Date::new_0().get_time();
    }

    pub fn update_tick_time(&mut self) {
        let now = Date::new_0().get_time();
        self.missed_time += (now - self.last_tick_time) - TICK_MS;
        // log::info!("delta: {}", (now - self.last_tick_time));
        // log::info!(
        //     "missed_time delta: {}",
        //     (now - self.last_tick_time) - TICK_MS
        // );

        self.last_tick_time = now;
    }
    pub fn convert_missed_time_to_saved_ticks(&mut self) {
        // log::info!("missed_time: {}", self.missed_time);
        let mut missed_ticks = self.missed_time_ticks();
        if missed_ticks >= 1.0 {
            missed_ticks = missed_ticks as u32 as f64;
            // log::info!("convert: {}", missed_ticks);
            self.saved_ticks += missed_ticks;
            self.missed_time -= missed_ticks * TICK_MS;
        }
    }

    pub fn missed_time_ticks(&mut self) -> f64 {
        self.missed_time / TICK_MS
    }

    pub fn skip_tick(&mut self) {
        self.missed_time += TICK_MS;
    }

    pub fn use_single_missed_time_tick(&mut self) {
        self.missed_time -= TICK_MS;
    }

    pub fn paused_tick_time(&mut self) {
        self.saved_ticks += 1.0;
        /*  commenting out next two lines because the if-block is empty,
            and 'now' is unused after commenting out the empty if-block

        let now = Date::new_0().get_time();
        if now >= self.last_tick_time + 500.0 {}
        */
        self.last_tick_time = Date::new_0().get_time();
    }
}

fn get_game_speed_default() -> u32 {
    if cfg!(debug_assertions) {
        10
    } else {
        1
    }
}

fn get_autosave_default() -> bool {
    !cfg!(debug_assertions)
}
