// use serde_json::json;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::console;

mod input;
mod state;
mod world;

use input::work::Work;
use input::Input;
use state::state_container::{new_game, StateContainer};
use world::world::World;

#[derive(Serialize, Deserialize)]
struct Game {
    input: Input,
    world: World,
    state: StateContainer,
}

impl Game {
    pub fn new() -> Game {
        let world = World::new();
        let state = new_game(&world);
        let input = Input::new();
        Game {
            world,
            state,
            input,
        }
    }
}

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
    let game = GLOBAL_DATA.lock().unwrap();
    console::log_1(&JsValue::from_serde(&game.state).unwrap());
    console::log_1(&JsValue::from_serde(&game.input).unwrap());
    console::log_1(&JsValue::from_serde(&game.world).unwrap());
}

#[wasm_bindgen]
pub fn work_mines() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.input.work = Work::Mines;
    console::log_1(&JsValue::from_str("Rust mines"));
}

#[wasm_bindgen]
pub fn work_fields() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.input.work = Work::Fields;
    console::log_1(&JsValue::from_str("Rust fields"));
}

#[wasm_bindgen]
pub fn work_servant() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.input.work = Work::Servant;
    console::log_1(&JsValue::from_str("Rust fields"));
}
