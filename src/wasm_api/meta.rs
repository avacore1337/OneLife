#![allow(dead_code)]

use crate::game::{Game, GameSave};
use crate::GLOBAL_DATA;
use libflate::gzip::{Decoder, Encoder};
use serde_json::{from_str, to_string};
use std::io::{Read, Write};
use std::str;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn set_disable_tutorial(val: bool) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.info.disable_tutorial = val;
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
    console::log_1(&JsValue::from_str("Resetting game"));
}

#[wasm_bindgen]
pub fn save() {
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    do_save(game);
}

pub fn do_save(game: &mut Game) {
    let window = web_sys::window().unwrap();
    console::log_1(&JsValue::from_str("Saving game"));
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