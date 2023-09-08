use crate::game::Game;
use crate::input::options::AutoSettingTypes;
use crate::GLOBAL_DATA;
use log::info;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn clear_recorded() {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    game.inputs.clear()
}

#[wasm_bindgen]
pub fn clear_previous_recorded() {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    game.previous_inputs.clear()
}

#[wasm_bindgen]
pub fn remove_recorded(val: u32) {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    if let Err(err) = game.inputs.remove(val) {
        info!("{:?}", err);
    }
}

#[wasm_bindgen]
pub fn remove_previous_recorded(val: u32) {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    if let Err(err) = game.previous_inputs.remove(val) {
        info!("{:?}", err);
    }
}

#[wasm_bindgen]
pub fn set_auto_end_early(val: f64) {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    set_auto_end_early_internal(val, game);
}

pub fn set_auto_end_early_internal(val: f64, game: &mut Game) {
    game.meta_data.options.auto_end_early_criteria = val;
    if val == 0.0 {
        //     game.register_input(AutoSettingTypes::AutoRebirthTrue)
        game.meta_data.options.auto_end_early = false;
    } else {
        game.meta_data.options.auto_end_early = true;
        //     game.register_input(AutoSettingTypes::AutoRebirthFalse)
    }
}

#[wasm_bindgen]
pub fn toggle_auto_rebirth() {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    set_auto_rebirth_internal(!game.meta_data.options.auto_rebirth, game);
}

#[wasm_bindgen]
pub fn set_auto_rebirth(val: bool) {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    set_auto_rebirth_internal(val, game);
}

pub fn set_auto_rebirth_internal(val: bool, game: &mut Game) {
    if val {
        game.register_input(AutoSettingTypes::AutoRebirthTrue)
    } else {
        game.register_input(AutoSettingTypes::AutoRebirthFalse)
    };
    game.meta_data.options.auto_rebirth = val;
}

#[wasm_bindgen]
pub fn toggle_auto_work() {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    set_auto_work_internal(!game.meta_data.options.auto_work, game);
}

#[wasm_bindgen]
pub fn set_auto_work(val: bool) {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    set_auto_work_internal(val, game);
}

pub fn set_auto_work_internal(val: bool, game: &mut Game) {
    if val {
        game.register_input(AutoSettingTypes::AutoWorkTrue)
    } else {
        game.register_input(AutoSettingTypes::AutoWorkFalse)
    };
    game.meta_data.options.auto_work = val;
}

#[wasm_bindgen]
pub fn toggle_auto_living() {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    set_auto_living_internal(!game.meta_data.options.auto_living, game);
}

#[wasm_bindgen]
pub fn set_auto_living(val: bool) {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    set_auto_living_internal(val, game);
}

pub fn set_auto_living_internal(val: bool, game: &mut Game) {
    if val {
        game.register_input(AutoSettingTypes::AutoLivingTrue)
    } else {
        game.register_input(AutoSettingTypes::AutoLivingFalse)
    };
    game.meta_data.options.auto_living = val;
}

#[wasm_bindgen]
pub fn toggle_auto_buy_blessing() {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    set_auto_buy_blessing_internal(!game.meta_data.options.auto_buy_blessing, game);
}

#[wasm_bindgen]
pub fn set_auto_buy_blessing(val: bool) {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    set_auto_buy_blessing_internal(val, game);
}

pub fn set_auto_buy_blessing_internal(val: bool, game: &mut Game) {
    if val {
        game.register_input(AutoSettingTypes::AutoBuyBlessingTrue)
    } else {
        game.register_input(AutoSettingTypes::AutoBuyBlessingFalse)
    };
    game.meta_data.options.auto_buy_blessing = val;
}

#[wasm_bindgen]
pub fn toggle_auto_buy_item() {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    set_auto_buy_item_internal(!game.meta_data.options.auto_buy_item, game);
}

#[wasm_bindgen]
pub fn set_auto_buy_item(val: bool) {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    set_auto_buy_item_internal(val, game);
}

pub fn set_auto_buy_item_internal(val: bool, game: &mut Game) {
    if val {
        game.register_input(AutoSettingTypes::AutoBuyItemTrue)
    } else {
        game.register_input(AutoSettingTypes::AutoBuyItemFalse)
    };
    game.meta_data.options.auto_buy_item = val;
}

#[wasm_bindgen]
pub fn toggle_auto_buy_tomb() {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    set_auto_buy_tomb_internal(!game.meta_data.options.auto_buy_tomb, game);
}

#[wasm_bindgen]
pub fn set_auto_buy_tomb(val: bool) {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    set_auto_buy_tomb_internal(val, game);
}

pub fn set_auto_buy_tomb_internal(val: bool, game: &mut Game) {
    if val {
        game.register_input(AutoSettingTypes::AutoBuyTombTrue)
    } else {
        game.register_input(AutoSettingTypes::AutoBuyTombFalse)
    };
    game.meta_data.options.auto_buy_tomb = val;
}
