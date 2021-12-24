use std::collections::BTreeMap;

use one_life::engine::engine_run;
use one_life::game::Game;
use one_life::input::skill::SkillTypes;
use one_life::input::work::WorkTypes;
use one_life::presets::make_t4;

use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_tactics() {
    let mut game = Game::new();
    let (state, _, _) = make_t4(&game.world);
    game.state = state;
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
fn test_input_conversion() {
    let data = r#" {"0":["Auto Work False","Auto Living False","Auto Buy Item False","Auto Buy Tomb False"]} "#;
    let inputs: BTreeMap<u32, Vec<String>> = serde_json::from_str(data).unwrap();
    let mut game = Game::new();
    let (state, _, _) = make_t4(&game.world);
    game.state = state;
    game.inputs = inputs;
}
