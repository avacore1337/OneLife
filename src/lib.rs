// use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::console;

mod engine;
mod game;
mod input;
mod state;
mod world;

use engine::engine_run;
use game::Game;

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
pub fn tick() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    engine_run(&mut game);
    let borrow: &Game = &game;
    console::log_1(&JsValue::from_serde(borrow).unwrap());
    console::log_1(&JsValue::from_serde(&game.state.items.coins).unwrap());
}

#[wasm_bindgen]
pub fn set_work(val: &JsValue) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.input.work = val.into_serde().unwrap();
    console::log_1(&JsValue::from_str("Rust generic work"));
}

#[wasm_bindgen]
pub fn hard_reset() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.hard_reset();
    console::log_1(&JsValue::from_str("Resetting game"));
}
