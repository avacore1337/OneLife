#![allow(dead_code)]

use crate::game::{Game, GameSave};
use crate::GLOBAL_DATA;
use libflate::gzip::{Decoder, Encoder};
use log::info;
use serde_json::{from_str, to_string};
use std::io::{Read, Write};
use std::str;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn set_update_rate(val: u32) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.options.update_rate = val;
}

#[wasm_bindgen]
pub fn set_disable_tutorial(val: bool) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.info.disable_tutorial = val;
}

#[wasm_bindgen]
pub fn toggle_paused() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.options.paused = !game.meta_data.options.paused;
}

#[wasm_bindgen]
pub fn set_paused(val: bool) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.options.paused = val;
}

#[wasm_bindgen]
pub fn toggle_show_recorded() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.options.show_recorded = !game.meta_data.options.show_recorded;
}

#[wasm_bindgen]
pub fn set_show_recorded(show: bool) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.options.show_recorded = show;
}

#[wasm_bindgen]
pub fn set_show_bought_items(show: bool) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.options.show_bought_items = show;
}

#[wasm_bindgen]
pub fn set_show_bought_upgrades(show: bool) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.options.show_bought_upgrades = show;
}

#[wasm_bindgen]
pub fn toggle_autosave() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.autosave = !game.meta_data.autosave;
}

#[wasm_bindgen]
pub fn set_autosave(autosave: bool) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.autosave = autosave;
}

#[wasm_bindgen]
pub fn hard_reset() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.hard_reset();
    info!("Resetting game");
}

#[wasm_bindgen]
pub fn save() {
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    do_save(game);
}

pub fn do_save(game: &mut Game) {
    let window = web_sys::window().unwrap();
    info!("Saving game");
    if let Ok(Some(local_storage)) = window.local_storage() {
        local_storage
            .set_item("save", &to_string(&GameSave::from(&*game)).unwrap())
            .unwrap();
        game.meta_data.set_save_time();
    }
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
            None => info!("You don't have a game to load"),
        }
    }
    info!("Loading game");
}

#[wasm_bindgen]
pub fn export_save() -> String {
    let game: &Game = &*GLOBAL_DATA.lock().unwrap();
    // let game = GLOBAL_DATA.lock().unwrap();
    info!("exporting game");
    let json_data = to_string(&GameSave::from(game)).unwrap();

    let mut encoder = Encoder::new(Vec::new()).unwrap();
    encoder.write_all(json_data.as_bytes()).unwrap();
    let res = encoder.finish().into_result().unwrap();
    let b64 = base64::encode(res);
    info!("{}", &b64);
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

    info!("{}", save_state);
    if let Ok(save) = from_str::<GameSave>(save_state) {
        current_game.load_game(save);
    }
}
