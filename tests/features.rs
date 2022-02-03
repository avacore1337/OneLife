use one_life::engine::{character_death_update, engine_run};
use one_life::game::Game;
use one_life::input::skill::SkillTypes;
use one_life::input::tomb::TombTypes;
use one_life::input::work::WorkTypes;
use one_life::input_recording::Inputs;
use one_life::presets::make_t4;
use one_life::tick_internal;
use one_life::wasm_api::auto_settings::set_auto_end_early_internal;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_tactics() {
    let mut game = Game::new();
    game.load_game(make_t4());
    game.input.work = WorkTypes::BagageBoy;

    let state2 = game.state.clone();

    engine_run(&mut game);
    let xp1 = game.state.works[WorkTypes::BagageBoy as usize].next_level_progress;

    game.state = state2;
    game.state.skills[SkillTypes::Tactics as usize].level = 50.0;
    engine_run(&mut game);
    let xp2 = game.state.works[WorkTypes::BagageBoy as usize].next_level_progress;

    assert!(xp2 > xp1 * 2.0);
}

#[wasm_bindgen_test]
fn test_auto_buy_tomb() {
    let mut game = Game::new();
    game.state.items.money = game.world.tombs[TombTypes::ShallowGrave as usize].purchasing_cost;
    game.meta_data.options.auto_buy_tomb = true;
    engine_run(&mut game); // Need to run once before for unlock calculation
    engine_run(&mut game);
    character_death_update(&mut game);
    assert_eq!(game.state.rebirth_stats.coins, 2.0);
}

#[wasm_bindgen_test]
fn test_input_conversion() {
    let data = r#" {"mapping":{"4540":[{"id":0,"name":"Set Housing FilthyBarracks"}],"5170":[{"id":1,"name":"Set Activity Studying"}]},"current_id":2} "#;
    let inputs: Inputs = serde_json::from_str(data).unwrap();
    let mut game = Game::new();
    game.inputs = inputs;
}

#[wasm_bindgen_test]
fn test_auto_end_early() {
    let mut game = Game::new();
    game.state.tombs[TombTypes::Mausuleum as usize].is_purchased = true;
    game.state.life_stats.current_tick = 5001;
    set_auto_end_early_internal(2.0, &mut game);
    tick_internal(&mut game); // Need to run once before for unlock calculation
    tick_internal(&mut game);
    assert_eq!(game.state.rebirth_stats.rebirth_count, 1);
    assert!(
        game.state.rebirth_stats.coins >= 2.0,
        "Coins {}",
        game.state.rebirth_stats.coins
    );
}
