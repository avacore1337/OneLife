#![feature(variant_count)]
// use serde_json::json;
use libflate::gzip::{Decoder, Encoder};
use once_cell::sync::Lazy;
use serde_json::{from_str, to_string};
use std::io::{Read, Write};
use std::str;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::console;

mod engine;
mod game;
mod input;
mod meta;
mod presets;
mod state;
mod world_content;

use engine::engine_run;
use game::{Game, GameSave};
use input::boost_item::BoostItemTypes;
use presets::get_presets;
use state::state_container::rebirth;
use world_content::boost_item::BoostItem;
use world_content::tier::Tier;

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
pub fn get_meta_data() -> JsValue {
    let game = GLOBAL_DATA.lock().unwrap();
    JsValue::from_serde(&game.meta_data).unwrap()
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
}

#[wasm_bindgen]
pub fn do_rebirth() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    if !game.state.life_stats.dead {
        return;
    }
    game.state.rebirth_stats.rebirth_count += 1;
    game.state = rebirth(&game.world, game.state.rebirth_stats.clone());
    console::log_1(&JsValue::from_str("Rust did rebirth"));
}

#[wasm_bindgen]
pub fn set_gamespeed(speed: u32) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.game_speed = speed;
}

#[wasm_bindgen]
pub fn set_autosave(autosave: bool) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.autosave = autosave;
}

#[wasm_bindgen]
pub fn print_debug() {
    let game = GLOBAL_DATA.lock().unwrap();
    console::log_1(&JsValue::from_str(&format!(
        "intermediate: {:?}",
        game.intermediate_state
    )));
    console::log_1(&JsValue::from_str(&format!("state: {:?}", game.state)));
}

#[wasm_bindgen]
pub fn tick() {
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    if game.meta_data.should_autosave() {
        do_save(game);
    }
    for _ in 0..game.meta_data.game_speed {
        engine_run(game);
    }
}

#[wasm_bindgen]
pub fn single_tick() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    engine_run(&mut game);
}

#[wasm_bindgen]
pub fn set_work(val: &JsValue) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.input.work = val.into_serde().unwrap();
    console::log_1(&JsValue::from_str("Rust set work"));
}

#[wasm_bindgen]
pub fn set_housing(val: &JsValue) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.input.housing = val.into_serde().unwrap();
    console::log_1(&JsValue::from_str("Rust set housing"));
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
    console::log_1(&JsValue::from_str("Rust buy tier"));
    if can_buy_tier(val) {
        console::log_1(&JsValue::from_str("Can buy tier"));
        let mut game = GLOBAL_DATA.lock().unwrap();
        let tier: &Tier = &game.world.tiers[val as usize];
        game.state.rebirth_stats.coins -= tier.purchasing_cost;
        game.state.rebirth_stats.class_tier = val;
    }
}

#[wasm_bindgen]
pub fn can_buy_item(val: &JsValue) -> bool {
    let game = GLOBAL_DATA.lock().unwrap();
    let boost_item_type: BoostItemTypes = val.into_serde().unwrap();
    let item: &BoostItem = &game.world.boost_items[boost_item_type as usize];
    // let next_tier: bool = game.state.rebirth_stats.class_tier + 1 == val;
    let can_afford: bool = game.state.items.money >= item.purchasing_cost;
    can_afford
}

#[wasm_bindgen]
pub fn buy_item(val: &JsValue) {
    let boost_item_type: BoostItemTypes = val.into_serde().unwrap();
    console::log_1(&JsValue::from_str("Rust buy item"));
    if can_buy_item(val) {
        console::log_1(&JsValue::from_str("Can buy item"));
        let mut game = GLOBAL_DATA.lock().unwrap();
        let item: &BoostItem = &game.world.boost_items[boost_item_type as usize];
        game.state.items.money -= item.purchasing_cost;
        game.state.items.boost_items[boost_item_type as usize].is_purchased = true;
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
    let game: &Game = &*GLOBAL_DATA.lock().unwrap();
    do_save(game);
}

pub fn do_save(game: &Game) {
    let window = web_sys::window().unwrap();
    if let Ok(Some(local_storage)) = window.local_storage() {
        local_storage
            .set_item("save", &to_string(&GameSave::from(game)).unwrap())
            .unwrap();
    }
    console::log_1(&JsValue::from_str("Saving game"));
}

#[wasm_bindgen]
pub fn load() {
    let mut current_game = GLOBAL_DATA.lock().unwrap();
    let window = web_sys::window().unwrap();
    if let Ok(Some(local_storage)) = window.local_storage() {
        match local_storage.get_item("save").unwrap() {
            Some(json_save) => {
                if let Ok(save) = from_str::<GameSave>(&json_save) {
                    current_game.load_game(save);
                }
            }
            None => console::log_1(&JsValue::from_str("You don't have a game to load")),
        }
    }
    console::log_1(&JsValue::from_str("Loading game"));
}

#[wasm_bindgen]
pub fn export_save() -> String {
    let game: &Game = &*GLOBAL_DATA.lock().unwrap();
    // let game = GLOBAL_DATA.lock().unwrap();
    console::log_1(&JsValue::from_str("exporting game"));
    let json_data = to_string(&GameSave::from(game)).unwrap();

    let mut encoder = Encoder::new(Vec::new()).unwrap();
    encoder.write_all(json_data.as_bytes()).unwrap();
    let res = encoder.finish().into_result().unwrap();
    let b64 = base64::encode(res);
    console::log_1(&JsValue::from_str(&b64));
    b64
}

#[wasm_bindgen]
pub fn import_save(save: String) {
    let mut current_game = GLOBAL_DATA.lock().unwrap();
    let data = base64::decode(save).unwrap();
    let mut decoder = Decoder::new(&data[..]).unwrap();
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data).unwrap();
    let save_state = match str::from_utf8(decoded_data.as_slice()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    console::log_1(&JsValue::from_str(save_state));
    if let Ok(save) = from_str::<GameSave>(save_state) {
        current_game.load_game(save);
    }
}

#[wasm_bindgen]
pub fn is_game_paused() -> bool {
    let game = GLOBAL_DATA.lock().unwrap();
    game.state.life_stats.dead
}
