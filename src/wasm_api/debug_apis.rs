#![allow(dead_code)]
#![cfg(debug_assertions)]

use crate::engine::engine_run;
// use crate::input_mapping::InputMapping;
use crate::presets::get_presets;
use log::info;
use wasm_bindgen::prelude::*;

use crate::GLOBAL_DATA;

#[wasm_bindgen]
pub fn grow_old() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.state.life_stats.age = game.state.life_stats.lifespan - 2.0;
}

#[wasm_bindgen]
pub fn give_money(money: f64) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.state.items.money = money;
}

#[wasm_bindgen]
pub fn give_coins(coins: f64) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.state.rebirth_stats.coins = coins;
}

#[wasm_bindgen]
pub fn print_debug_intermediate() {
    let game = GLOBAL_DATA.lock().unwrap();
    info!("intermediate: {:#?}", game.intermediate_state);
}

#[wasm_bindgen]
pub fn print_debug_state() {
    let game = GLOBAL_DATA.lock().unwrap();
    info!("state: {:#?}", game.state);
}

#[wasm_bindgen]
pub fn print_debug_meta() {
    let game = GLOBAL_DATA.lock().unwrap();
    info!("meta: {:#?}", game.meta_data);
}

#[wasm_bindgen]
pub fn single_tick() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    engine_run(&mut game);
}

#[wasm_bindgen]
pub fn set_gamespeed(speed: u32) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.game_speed = speed;
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
    if let Some((state, input, meta_data)) = presets.remove(preset_name) {
        game.state = state;
        game.input = input;
        game.meta_data = meta_data
    }
}

#[wasm_bindgen]
pub fn test() {
    // let user_input_mapping = InputMapping::default();
    // info!("Mapping: {:#?}", user_input_mapping.user_function.keys());
    let game = GLOBAL_DATA.lock().unwrap();
    info!("input: {:#?}", game.inputs);
}
