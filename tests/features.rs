use std::collections::BTreeMap;

use one_life::engine::{character_death_update, engine_run};
use one_life::game::Game;
use one_life::input::skill::SkillTypes;
use one_life::input::tomb::TombTypes;
use one_life::input::work::WorkTypes;
use one_life::presets::make_t4;

use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_tactics() {
    let mut game = Game::new();
    game.load_game(make_t4(&game.world));
    game.input.work = WorkTypes::BagageBoy;

    let state2 = game.state.clone();

    engine_run(&mut game);
    let xp1 = game.state.works[WorkTypes::BagageBoy as usize].next_level_progress;

    game.state = state2;
    game.state.skills[SkillTypes::Tactics as usize].level = 50.0;
    engine_run(&mut game);
    let xp2 = game.state.works[WorkTypes::BagageBoy as usize].next_level_progress;

    assert!(xp2 > xp1 * 2.0);
    // game2.state.works[WorkTypes
}

#[wasm_bindgen_test]
fn test_auto_buy_tomb() {
    let mut game = Game::new();
    game.state.items.money = game.world.tombs[TombTypes::ShallowGrave as usize].purchasing_cost;
    game.meta_data.options.auto_buy_tomb = true;
    engine_run(&mut game);
    character_death_update(&mut game);
    assert_eq!(game.state.rebirth_stats.coins, 2.0);
}

#[wasm_bindgen_test]
fn test_input_conversion() {
    let data = r#" {"0":["Auto Work False","Auto Living False","Auto Buy Item False","Auto Buy Tomb False"]} "#;
    let inputs: BTreeMap<u32, Vec<String>> = serde_json::from_str(data).unwrap();
    let mut game = Game::new();
    game.inputs = inputs;
}
