#![allow(dead_code)]
#![cfg(debug_assertions)]

use crate::engine::engine_run;
use crate::presets::get_presets;
use log::info;
use wasm_bindgen::prelude::*;

use crate::GLOBAL_DATA;

#[wasm_bindgen]
pub fn print_debug_state() {
    let game = GLOBAL_DATA.lock().unwrap();
    info!("intermediate: {:#?}", game.intermediate_state);
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
