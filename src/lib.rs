// use serde_json::json;
use once_cell::sync::Lazy;
use serde_json::{from_str, to_string};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::console;

mod engine;
mod game;
mod input;
mod presets;
mod state;
mod world;

use engine::engine_run;
use game::Game;
use input::Input;
use presets::get_presets;
use state::state_container::{rebirth, StateContainer};
use world::tier::Tier;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static GLOBAL_DATA: Lazy<Mutex<Game>> = Lazy::new(|| Mutex::new(Game::new()));

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello One Life!"));

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, one-life!");
}

#[wasm_bindgen]
pub fn get_world() -> JsValue {
    let game = GLOBAL_DATA.lock().unwrap();
    JsValue::from_serde(&game.world).unwrap()
}

#[wasm_bindgen]
pub fn get_input() -> JsValue {
    let game = GLOBAL_DATA.lock().unwrap();
    JsValue::from_serde(&game.input).unwrap()
}

#[wasm_bindgen]
pub fn get_state() -> JsValue {
    let game = GLOBAL_DATA.lock().unwrap();
    JsValue::from_serde(&game.state).unwrap()
}

#[wasm_bindgen]
pub fn get_preset_saves() -> JsValue {
    let game = GLOBAL_DATA.lock().unwrap();
    JsValue::from_serde(&get_presets(&game.world)).unwrap()
}

#[wasm_bindgen]
pub fn set_preset_saves(preset_name: &str) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    let mut presets = get_presets(&game.world);
    if let Some((state, input)) = presets.remove(preset_name) {
        game.state = state;
        game.input = input;
    }
    // game.input.housing = val.into_serde().unwrap();
    // console::log_1(&JsValue::from_str("Rust generic work"));
}

#[wasm_bindgen]
pub fn do_rebirth() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.state.rebirth_stats.rebirth_count += 1;
    game.state = rebirth(&game.world, game.state.rebirth_stats.clone());
}

#[wasm_bindgen]
pub fn tick() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    engine_run(&mut game, 100.0);
    // let borrow: &Game = &game;
    // console::log_1(&JsValue::from_serde(borrow).unwrap());
    // console::log_1(&JsValue::from_serde(&game.state.items.money).unwrap());
    // console::log_1(&JsValue::from_serde(&game.state.life_stats.age).unwrap());
    // console::log_1(&JsValue::from_serde(&game.state.life_stats.dead).unwrap());
}

#[wasm_bindgen]
pub fn set_work(val: &JsValue) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.input.work = val.into_serde().unwrap();
    console::log_1(&JsValue::from_str("Rust generic work"));
}

#[wasm_bindgen]
pub fn set_housing(val: &JsValue) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.input.housing = val.into_serde().unwrap();
    console::log_1(&JsValue::from_str("Rust generic work"));
}

#[wasm_bindgen]
pub fn can_buy_tier(val: u32) -> bool {
    let game = GLOBAL_DATA.lock().unwrap();
    let tier: &Tier = &game.world.tiers[val as usize];
    let next_tier: bool = game.state.rebirth_stats.class_tier + 1 == val;
    let can_afford: bool = game.state.rebirth_stats.coins >= tier.purchasing_cost;
    can_afford && next_tier
}

#[wasm_bindgen]
pub fn buy_tier(val: u32) {
    console::log_1(&JsValue::from_str("Buy tier"));
    if can_buy_tier(val) {
        console::log_1(&JsValue::from_str("Can buy tier"));
        let mut game = GLOBAL_DATA.lock().unwrap();
        let tier: &Tier = &game.world.tiers[val as usize];
        game.state.rebirth_stats.coins -= tier.purchasing_cost;
        game.state.rebirth_stats.class_tier = val;
    }
}

#[wasm_bindgen]
pub fn hard_reset() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.hard_reset();
    console::log_1(&JsValue::from_str("Resetting game"));
}

#[wasm_bindgen]
pub fn save() {
    let game = GLOBAL_DATA.lock().unwrap();
    let window = web_sys::window().unwrap();
    if let Ok(Some(local_storage)) = window.local_storage() {
        local_storage
            .set_item("gamestate", &to_string(&game.state).unwrap())
            .unwrap();
        local_storage
            .set_item("input", &to_string(&game.input).unwrap())
            .unwrap();
    }
    console::log_1(&JsValue::from_str("Saving game"));
}

#[wasm_bindgen]
pub fn load() {
    let mut current_game = GLOBAL_DATA.lock().unwrap();
    let window = web_sys::window().unwrap();
    if let Ok(Some(local_storage)) = window.local_storage() {
        match local_storage.get_item("gamestate").unwrap() {
            Some(json_state) => {
                if let Ok(state) = from_str::<StateContainer>(&json_state) {
                    current_game.state = state;
                }
            }
            None => console::log_1(&JsValue::from_str("You don't have a game to load")),
        }
        match local_storage.get_item("input").unwrap() {
            Some(json_input) => {
                if let Ok(input) = from_str::<Input>(&json_input) {
                    current_game.input = input;
                }
            }
            None => console::log_1(&JsValue::from_str("You don't have a game to load")),
        }
    }
    console::log_1(&JsValue::from_str("Loading game"));
}

#[wasm_bindgen]
pub fn is_game_paused() -> bool {
    let game = GLOBAL_DATA.lock().unwrap();
    game.state.life_stats.dead
}
