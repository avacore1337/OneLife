#![allow(non_upper_case_globals)]

use icon::{Icon, IconType};
use input_recording::{Inputs, RecordedInputEntry};
use log::{info, Level};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate lazy_static;

// #[macro_use]
// extern crate serde_big_array;

pub mod engine;
pub mod game;
pub mod icon;
pub mod info;
pub mod input;
pub mod input_mapping;
pub mod input_recording;
pub mod meta;
pub mod presets;
pub mod state;
pub mod util;
pub mod wasm_api;
pub mod world_content;

use crate::info::check_for_tutorial_step;
use crate::input::blessing::BlessingTypes;
use engine::auto_functions::register_auto_settings;
use engine::{character_death_update, engine_run, update_unlocks};
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
use world_content::world::World;

const BASE_LIFESPAN: f64 = 70.0 * 365.0;
const TICK_RATE: f64 = 30.0;
const TICK_MS: f64 = 1000.0 / TICK_RATE;

lazy_static! {
    static ref WORLD: World = World::default();
    static ref GLOBAL_DATA: Mutex<Game> = Mutex::new(Game::new());
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_log::init_with_level(Level::Info).expect("error initializing log");

    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    set_panic_hook();

    // Your code goes here!
    info!("Hello One Life!");

    Ok(())
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn get_icon_by_enum(val: JsValue) -> JsValue {
    let icon_type: IconType = serde_wasm_bindgen::from_value(val).unwrap();
    let icon: Icon = icon_type.into();
    serde_wasm_bindgen::to_value(&icon).unwrap()
}

#[wasm_bindgen]
pub fn get_world_item_queue() -> JsValue {
    let game = GLOBAL_DATA.lock().unwrap();
    serde_wasm_bindgen::to_value(&game.input.get_world_item_queue()).unwrap()
}

#[wasm_bindgen]
pub fn get_recorded_inputs() -> JsValue {
    let game = GLOBAL_DATA.lock().unwrap();
    serde_wasm_bindgen::to_value(&Into::<Vec<RecordedInputEntry>>::into(&game.inputs)).unwrap()
}

#[wasm_bindgen]
pub fn get_previous_recorded_inputs() -> JsValue {
    let game = GLOBAL_DATA.lock().unwrap();
    serde_wasm_bindgen::to_value(&Into::<Vec<RecordedInputEntry>>::into(
        &game.previous_inputs,
    ))
    .unwrap()
}

#[wasm_bindgen]
pub fn get_world() -> JsValue {
    let game = GLOBAL_DATA.lock().unwrap();
    serde_wasm_bindgen::to_value(&game.world).unwrap()
}

#[wasm_bindgen]
pub fn get_input() -> JsValue {
    let game = GLOBAL_DATA.lock().unwrap();
    serde_wasm_bindgen::to_value(&game.input).unwrap()
}

#[wasm_bindgen]
pub fn get_state() -> JsValue {
    let game = GLOBAL_DATA.lock().unwrap();
    // This method is the standard one and might be faster or slower?
    // serde_wasm_bindgen::to_value(&game.state).unwrap()
    serde_wasm_bindgen::to_value(&game.state).unwrap()
}

#[wasm_bindgen]
pub fn get_meta_data() -> JsValue {
    let game = GLOBAL_DATA.lock().unwrap();
    serde_wasm_bindgen::to_value(&game.meta_data).unwrap()
}

#[wasm_bindgen]
pub fn next_info_step() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.info.tutorial_step.increment();
    game.meta_data.info.show_tutorial = false;
}

#[wasm_bindgen]
pub fn get_completed_steps() -> JsValue {
    let game = GLOBAL_DATA.lock().unwrap();
    serde_wasm_bindgen::to_value(&game.meta_data.info.get_completed_steps()).unwrap()
}

#[wasm_bindgen]
pub fn do_rebirth() {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    if !game.state.life_stats.dead {
        return;
    }
    do_rebirth_internal(game);
    info!("Rust did rebirth");
}

pub fn do_rebirth_internal(game: &mut Game) {
    game.state.rebirth_stats.rebirth_count += 1;
    game.state = rebirth(game.state.rebirth_stats.clone());
    game.input = Input::new(&game.state);
    game.previous_inputs = game.inputs.clone();
    game.inputs = Inputs::default();
    register_auto_settings(game);
    engine_run(game);
}

#[wasm_bindgen]
pub fn do_rebirth_replay() {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    if !game.state.life_stats.dead {
        return;
    }
    do_rebirth_internal(game);
    game.state.life_stats.replaying = true;
    info!("Rust did rebirth replay");
}

#[wasm_bindgen]
pub fn paused() {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    check_for_tutorial_step(game);
    game.meta_data.update_tick_time();
    game.meta_data.skip_tick();
    game.meta_data.convert_missed_time_to_saved_ticks();
    if game.meta_data.options.auto_rebirth {
        die_internal(game);
        do_rebirth_internal(game);
    }
}

#[wasm_bindgen]
pub fn toggle_use_saved_ticks() {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    game.meta_data.use_saved_ticks = !game.meta_data.use_saved_ticks;
}

#[wasm_bindgen]
pub fn use_saved_ticks(val: bool) {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    game.meta_data.use_saved_ticks = val;
}

#[wasm_bindgen]
pub fn tick() {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    tick_internal(game);
}

pub fn tick_internal(game: &mut Game) {
    check_for_tutorial_step(game);
    if game.meta_data.should_autosave() {
        do_save(game);
    }
    game.meta_data.update_tick_time();
    if game.just_loaded {
        game.just_loaded = false;
        if game.state.life_stats.is_dying {
            return;
        }
        one_tick(game);
        game.meta_data.convert_missed_time_to_saved_ticks();
    } else {
        if game.state.life_stats.is_dying {
            return;
        }
        if game.meta_data.missed_time_ticks() < -1.0 {
            game.meta_data.skip_tick();
            return;
        }
        one_tick(game);
        handle_missing_time(game);
    }
}

pub fn handle_missing_time(game: &mut Game) {
    if game.meta_data.options.use_missed_ticks {
        for i in 0..(game.meta_data.missed_time_ticks() as u64) {
            if i > game.meta_data.options.max_missed_ticks as u64 {
                game.meta_data.convert_missed_time_to_saved_ticks();
                return;
            }
            if game.state.life_stats.is_dying {
                return;
            }
            game.meta_data.use_single_missed_time_tick();
            one_tick(game);
        }
    } else {
        game.meta_data.convert_missed_time_to_saved_ticks();
    }
}

pub fn one_tick(game: &mut Game) {
    for _ in 0..game.meta_data.game_speed {
        engine_run(game);
    }
    if should_auto_end_early(game) {
        info!("Auto ending early");
        die_internal(game);
        do_rebirth_internal(game);
    }
}

fn should_auto_end_early(game: &Game) -> bool {
    let options = &game.meta_data.options;
    let enough_coins = options.auto_end_early_criteria < game.state.rebirth_stats.coins_gain;
    let old_enough = game.state.life_stats.current_tick >= 5000;
    // info!(
    //     "Auto ending early {}, enough_coins {}, criteria {}, gain {}",
    //     options.auto_end_early,
    //     enough_coins,
    //     options.auto_end_early_criteria,
    //     game.state.rebirth_stats.coins_gain
    // );
    options.auto_end_early && enough_coins && old_enough
}

#[wasm_bindgen]
pub fn set_work(val: &JsValue) {
    info!("Rust set work");
    let work_type = serde_wasm_bindgen::from_value(val.clone()).unwrap();
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    set_work_internal(work_type, game);
}

pub fn set_work_internal(work_type: WorkTypes, game: &mut Game) {
    game.register_input(work_type);
    game.input.work = work_type;
}

#[wasm_bindgen]
pub fn set_housing(val: &JsValue) {
    info!("Rust set housing");
    let housing_type = serde_wasm_bindgen::from_value(val.clone()).unwrap();
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    set_housing_internal(housing_type, game);
}

pub fn set_housing_internal(housing_type: HousingTypes, game: &mut Game) {
    game.register_input(housing_type);
    game.input.housing = housing_type;
}

#[wasm_bindgen]
pub fn set_activity(val: &JsValue) {
    info!("Rust set activity");
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    let activity_type = serde_wasm_bindgen::from_value(val.clone()).unwrap();
    set_activity_internal(activity_type, game);
}

pub fn set_activity_internal(activity_type: ActivityTypes, game: &mut Game) {
    game.register_input(activity_type);
    game.input.activity = activity_type;
}

#[wasm_bindgen]
pub fn can_buy_tier(val: u32) -> bool {
    let game = GLOBAL_DATA.lock().unwrap();
    let tier: &Tier = &game.world.tiers[val as usize];
    let next_tier: bool = game.state.rebirth_stats.tier + 1 == val;
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
        game.state.rebirth_stats.tier = val;
        update_unlocks(&mut game);
    }
}

#[wasm_bindgen]
pub fn buy_tomb(val: &JsValue) {
    info!("Rust buy tomb");
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    let tomb_type: TombTypes = serde_wasm_bindgen::from_value(val.clone()).unwrap();
    buy_tomb_internal(tomb_type, game);
}

pub fn buy_tomb_internal(tomb_type: TombTypes, game: &mut Game) {
    if can_buy_tomb(tomb_type, game) {
        info!("Can buy tomb");
        game.register_input(tomb_type);

        let tomb: &Tomb = &game.world.tombs[tomb_type as usize];
        game.state.items.money -= tomb.purchasing_cost;
        game.state.tombs[tomb_type as usize].is_purchased = true;
        update_unlocks(game);
    }
}

pub fn can_buy_tomb(tomb_type: TombTypes, game: &mut Game) -> bool {
    let tomb = &game.world.tombs[tomb_type as usize];
    let tomb_state = &game.state.tombs[tomb_type as usize];
    let can_afford: bool = game.state.items.money >= tomb.purchasing_cost;
    can_afford && !tomb_state.is_purchased
}

#[wasm_bindgen]
pub fn buy_blessing(val: &JsValue) {
    info!("Rust buy item");
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    let boost_item_type: BlessingTypes = serde_wasm_bindgen::from_value(val.clone()).unwrap();
    buy_blessing_internal(boost_item_type, game);
}

pub fn buy_blessing_internal(blessing_type: BlessingTypes, game: &mut Game) {
    if can_buy_blessing(blessing_type, game) {
        info!("Can buy blessing");
        game.register_input(blessing_type);
        let blessing = &game.state.blessings[blessing_type as usize];
        game.state.items.divine_favor -= blessing.next_level_cost;
        game.state.blessings[blessing_type as usize].level += 1;
    }
}

pub fn can_buy_blessing(blessing: BlessingTypes, game: &mut Game) -> bool {
    let blessing = &game.state.blessings[blessing as usize];
    let can_afford: bool = game.state.items.divine_favor >= blessing.next_level_cost;
    can_afford
}

#[wasm_bindgen]
pub fn dequeue_item(val: &JsValue) {
    info!("Rust dequeue item");
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    let boost_item_type: BoostItemTypes = serde_wasm_bindgen::from_value(val.clone()).unwrap();
    game.input.dequeue_item(boost_item_type);
}

#[wasm_bindgen]
pub fn queue_item(val: &JsValue) {
    info!("Rust queue item");
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    let boost_item_type: BoostItemTypes = serde_wasm_bindgen::from_value(val.clone()).unwrap();
    game.input.queue_item(boost_item_type);
}

#[wasm_bindgen]
pub fn buy_item(val: &JsValue) {
    info!("Rust buy item");
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    let boost_item_type: BoostItemTypes = serde_wasm_bindgen::from_value(val.clone()).unwrap();
    buy_item_internal(boost_item_type, game);
}

pub fn buy_item_internal(boost_item_type: BoostItemTypes, game: &mut Game) {
    if can_buy_item(boost_item_type, game) {
        info!("Can buy item");
        game.register_input(boost_item_type);
        let item: &BoostItem = &game.world.boost_items[boost_item_type as usize];
        game.state.items.money -= item.purchasing_cost;
        game.state.boost_items[boost_item_type as usize].is_purchased = true;
        update_unlocks(game);
    }
}

pub fn can_buy_item(boost_item_type: BoostItemTypes, game: &mut Game) -> bool {
    let item = &game.world.boost_items[boost_item_type as usize];
    let item_state = &game.state.boost_items[boost_item_type as usize];
    let can_afford: bool = game.state.items.money >= item.purchasing_cost;
    can_afford && !item_state.is_purchased
}

#[wasm_bindgen]
pub fn buy_rebirth_upgrade(val: &JsValue) {
    let rebirth_upgrade_type: RebirthUpgradeTypes =
        serde_wasm_bindgen::from_value(val.clone()).unwrap();
    info!("Rust buy rebirth upgrade");
    if true {
        info!("Can buy rebirth upgrade");
        let mut game = GLOBAL_DATA.lock().unwrap();
        let rebirth_upgrade: &RebirthUpgrade =
            &game.world.rebirth_upgrades[rebirth_upgrade_type as usize];
        game.state.rebirth_stats.coins -= rebirth_upgrade.purchasing_cost;
        game.state.rebirth_stats.rebirth_upgrades[rebirth_upgrade_type as usize].is_purchased =
            true;
        update_unlocks(&mut game);
    }
}

#[wasm_bindgen]
pub fn die() {
    let game: &mut Game = &mut GLOBAL_DATA.lock().unwrap();
    info!("dying");
    die_internal(game)
}

fn die_internal(game: &mut Game) {
    engine_run(game);
    character_death_update(game);
    update_unlocks(&mut *game);
}
