#![feature(variant_count)]
use input::housing::HousingTypes;
use input::work::WorkTypes;
// use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::console;

const BASE_LIFESPAN: f64 = 70.0 * 365.0;
const TICK_RATE: f64 = 30.0;

mod engine;
mod experiment;
mod game;
mod info;
mod input;
mod meta;
mod presets;
mod state;
mod wasm_api;
mod world_content;

use engine::{character_death_update, engine_run};
use game::Game;
use input::boost_item::BoostItemTypes;
use input::rebirth_upgrade::RebirthUpgradeTypes;
use input::tomb::TombTypes;
use input::Input;
use state::state_container::rebirth;
use wasm_api::meta::do_save;
use world_content::boost_item::BoostItem;
use world_content::rebirth_upgrade::RebirthUpgrade;
use world_content::tier::Tier;
use world_content::tomb::Tomb;

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
pub fn next_info_step() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.info.tutorial_step += 1;
    game.meta_data.info.show_tutorial = false;
}

#[wasm_bindgen]
pub fn do_rebirth() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    if !game.state.life_stats.dead {
        return;
    }
    game.state.rebirth_stats.rebirth_count += 1;
    game.state = rebirth(&game.world, game.state.rebirth_stats.clone());
    game.input = Input::new(&game.state, &game.world);
    console::log_1(&JsValue::from_str("Rust did rebirth"));
}

#[wasm_bindgen]
pub fn do_rebirth_replay() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    if !game.state.life_stats.dead {
        return;
    }
    game.state.rebirth_stats.rebirth_count += 1;
    game.state = rebirth(&game.world, game.state.rebirth_stats.clone());
    game.input = Input::new(&game.state, &game.world);
    game.state.life_stats.replaying = true;
    console::log_1(&JsValue::from_str("Rust did rebirth replay"));
}

#[wasm_bindgen]
pub fn paused() {
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    game.meta_data.paused_tick_time();
}

#[wasm_bindgen]
pub fn set_auto_work(val: bool) {
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    game.input.options.auto_work = val;
}

#[wasm_bindgen]
pub fn set_auto_living(val: bool) {
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    game.input.options.auto_living = val;
}

#[wasm_bindgen]
pub fn set_auto_buy_item(val: bool) {
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    game.input.options.auto_buy_item = val;
}

#[wasm_bindgen]
pub fn use_saved_ticks(val: bool) {
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    game.meta_data.use_saved_ticks = val;
}

#[wasm_bindgen]
pub fn tick() {
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    if game.meta_data.should_autosave() {
        do_save(game);
    }
    game.meta_data.update_tick_time();
    for _ in 0..game.meta_data.game_speed {
        engine_run(game);
    }
}

#[wasm_bindgen]
pub fn set_work(val: &JsValue) {
    console::log_1(&JsValue::from_str("Rust set work"));
    let work_type = val.into_serde().unwrap();
    set_work_internal(work_type);
}

pub fn set_work_internal(work_type: WorkTypes) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    let name: String = format!("Set Work {:#?}", work_type);
    game.register_input(name);
    game.input.work = work_type;
}

#[wasm_bindgen]
pub fn set_housing(val: &JsValue) {
    console::log_1(&JsValue::from_str("Rust set housing"));
    let housing_type = val.into_serde().unwrap();
    set_housing_internal(housing_type);
}

pub fn set_housing_internal(housing_type: HousingTypes) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    let name: String = format!("Set Housing {:#?}", housing_type);
    game.register_input(name);
    game.input.housing = housing_type;
}

#[wasm_bindgen]
pub fn set_activity(val: &JsValue) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.input.activity = val.into_serde().unwrap();
    console::log_1(&JsValue::from_str("Rust set activity"));
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

pub fn can_buy_tomb(tomb_type: TombTypes) -> bool {
    let game = GLOBAL_DATA.lock().unwrap();
    let tomb: &Tomb = &game.world.tombs[tomb_type as usize];
    let can_afford: bool = game.state.items.money >= tomb.purchasing_cost;
    can_afford
}

#[wasm_bindgen]
pub fn buy_tomb(val: &JsValue) {
    let tomb_type: TombTypes = val.into_serde().unwrap();
    console::log_1(&JsValue::from_str("Rust buy tomb"));
    buy_tomb_internal(tomb_type);
}

pub fn buy_tomb_internal(tomb_type: TombTypes) {
    if can_buy_tomb(tomb_type) {
        console::log_1(&JsValue::from_str("Can buy tomb"));
        let mut game = GLOBAL_DATA.lock().unwrap();
        let name: String = format!("Buy Tomb {:#?}", tomb_type);
        game.register_input(name);

        let tomb: &Tomb = &game.world.tombs[tomb_type as usize];
        game.state.items.money -= tomb.purchasing_cost;
        game.state.tombs[tomb_type as usize].is_purchased = true;
    }
}

#[wasm_bindgen]
pub fn buy_item(val: &JsValue) {
    let boost_item_type: BoostItemTypes = val.into_serde().unwrap();
    console::log_1(&JsValue::from_str("Rust buy item"));
    buy_item_internal(boost_item_type);
}

pub fn buy_item_internal(boost_item_type: BoostItemTypes) {
    if can_buy_item(boost_item_type) {
        console::log_1(&JsValue::from_str("Can buy item"));
        let mut game = GLOBAL_DATA.lock().unwrap();
        let name: String = format!("Buy Item {:#?}", boost_item_type);
        game.register_input(name);
        let item: &BoostItem = &game.world.boost_items[boost_item_type as usize];
        game.state.items.money -= item.purchasing_cost;
        game.state.items.boost_items[boost_item_type as usize].is_purchased = true;
    }
}

pub fn can_buy_item(boost_item_type: BoostItemTypes) -> bool {
    let game = GLOBAL_DATA.lock().unwrap();
    let item: &BoostItem = &game.world.boost_items[boost_item_type as usize];
    let can_afford: bool = game.state.items.money >= item.purchasing_cost;
    can_afford
}

#[wasm_bindgen]
pub fn buy_rebirth_upgrade(val: &JsValue) {
    let rebirth_upgrade_type: RebirthUpgradeTypes = val.into_serde().unwrap();
    console::log_1(&JsValue::from_str("Rust buy rebirth upgrade"));
    if true {
        console::log_1(&JsValue::from_str("Can buy rebirth upgrade"));
        let mut game = GLOBAL_DATA.lock().unwrap();
        let rebirth_upgrade: &RebirthUpgrade =
            &game.world.rebirth_upgrades[rebirth_upgrade_type as usize];
        game.state.rebirth_stats.coins -= rebirth_upgrade.purchasing_cost;
        game.state.rebirth_stats.rebirth_upgrades[rebirth_upgrade_type as usize].is_purchased =
            true;
    }
}

#[wasm_bindgen]
pub fn die() {
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    console::log_1(&JsValue::from_str("dying"));
    character_death_update(game);
}

// #[wasm_bindgen]
// #[cfg(not(debug_assertions))]
// pub fn get_preset_saves() -> JsValue {
//     let v: Vec<u64> = vec![];
//     JsValue::from_serde(&v).unwrap()
// }
