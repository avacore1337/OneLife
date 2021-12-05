#![feature(variant_count)]
use log::{info, Level};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

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
use input::activity::ActivityTypes;
use input::boost_item::BoostItemTypes;
use input::housing::HousingTypes;
use input::rebirth_upgrade::RebirthUpgradeTypes;
use input::tomb::TombTypes;
use input::work::WorkTypes;
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
    console_log::init_with_level(Level::Info).expect("error initializing log");
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    info!("Hello One Life!");

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
    info!("Rust did rebirth");
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
    info!("Rust did rebirth replay");
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
    info!("Rust set work");
    let work_type = val.into_serde().unwrap();
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    set_work_internal(work_type, game);
}

pub fn set_work_internal(work_type: WorkTypes, game: &mut Game) {
    let name: String = format!("Set Work {:#?}", work_type);
    game.register_input(name);
    game.input.work = work_type;
}

#[wasm_bindgen]
pub fn set_housing(val: &JsValue) {
    info!("Rust set housing");
    let housing_type = val.into_serde().unwrap();
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    set_housing_internal(housing_type, game);
}

pub fn set_housing_internal(housing_type: HousingTypes, game: &mut Game) {
    let name: String = format!("Set Housing {:#?}", housing_type);
    game.register_input(name);
    game.input.housing = housing_type;
}

#[wasm_bindgen]
pub fn set_activity(val: &JsValue) {
    info!("Rust set activity");
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    let activity_type = val.into_serde().unwrap();
    set_activity_internal(activity_type, game);
}

pub fn set_activity_internal(activity_type: ActivityTypes, game: &mut Game) {
    let name: String = format!("Set Activity {:#?}", activity_type);
    game.register_input(name);
    game.input.activity = activity_type;
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
    info!("Rust buy tier");
    if can_buy_tier(val) {
        info!("Can buy tier");
        let mut game = GLOBAL_DATA.lock().unwrap();
        let tier: &Tier = &game.world.tiers[val as usize];
        game.state.rebirth_stats.coins -= tier.purchasing_cost;
        game.state.rebirth_stats.class_tier = val;
    }
}

#[wasm_bindgen]
pub fn buy_tomb(val: &JsValue) {
    info!("Rust buy tomb");
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    let tomb_type: TombTypes = val.into_serde().unwrap();
    buy_tomb_internal(tomb_type, game);
}

pub fn buy_tomb_internal(tomb_type: TombTypes, game: &mut Game) {
    if can_buy_tomb(tomb_type, game) {
        info!("Can buy tomb");
        let name: String = format!("Buy Tomb {:#?}", tomb_type);
        game.register_input(name);

        let tomb: &Tomb = &game.world.tombs[tomb_type as usize];
        game.state.items.money -= tomb.purchasing_cost;
        game.state.tombs[tomb_type as usize].is_purchased = true;
    }
}

pub fn can_buy_tomb(tomb_type: TombTypes, game: &mut Game) -> bool {
    let tomb: &Tomb = &game.world.tombs[tomb_type as usize];
    let can_afford: bool = game.state.items.money >= tomb.purchasing_cost;
    can_afford
}

#[wasm_bindgen]
pub fn buy_item(val: &JsValue) {
    info!("Rust buy item");
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    let boost_item_type: BoostItemTypes = val.into_serde().unwrap();
    buy_item_internal(boost_item_type, game);
}

pub fn buy_item_internal(boost_item_type: BoostItemTypes, game: &mut Game) {
    if can_buy_item(boost_item_type, game) {
        info!("Can buy item");
        let name: String = format!("Buy Item {:#?}", boost_item_type);
        game.register_input(name);
        let item: &BoostItem = &game.world.boost_items[boost_item_type as usize];
        game.state.items.money -= item.purchasing_cost;
        game.state.items.boost_items[boost_item_type as usize].is_purchased = true;
    }
}

pub fn can_buy_item(boost_item_type: BoostItemTypes, game: &mut Game) -> bool {
    let item: &BoostItem = &game.world.boost_items[boost_item_type as usize];
    let can_afford: bool = game.state.items.money >= item.purchasing_cost;
    can_afford
}

#[wasm_bindgen]
pub fn buy_rebirth_upgrade(val: &JsValue) {
    let rebirth_upgrade_type: RebirthUpgradeTypes = val.into_serde().unwrap();
    info!("Rust buy rebirth upgrade");
    if true {
        info!("Can buy rebirth upgrade");
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
    info!("dying");
    character_death_update(game);
}

// #[wasm_bindgen]
// #[cfg(not(debug_assertions))]
// pub fn get_preset_saves() -> JsValue {
//     let v: Vec<u64> = vec![];
//     JsValue::from_serde(&v).unwrap()
// }
