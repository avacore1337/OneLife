// #![cfg(target_arch = "wasm32")]

use one_life::game::Game;
// use one_life::input::options::AutoSettingTypes;
use one_life::input::work::WorkTypes;
use one_life::presets::elevent_rebirth;

use one_life::util::run_until_dead;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_eleveth_rebirth() {
    let game = &mut Game::new();
    game.load_game(elevent_rebirth());
    run_until_dead(game);
    assert_eq!(game.state.works[WorkTypes::Peltasts as usize].level, 10); // too strict?
}
