// #![cfg(target_arch = "wasm32")]

use one_life::game::Game;
use one_life::input::tomb::TombTypes;
// use one_life::input::options::AutoSettingTypes;
use one_life::input::work::WorkTypes;
use one_life::presets::{tenth_rebirth, third_rebirth};

use one_life::util::{do_test_rebirth, run_until_dead};
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_third_rebirth() {
    let game = &mut Game::new();
    game.load_game(third_rebirth());
    run_until_dead(game);
    assert_eq!(game.state.works[WorkTypes::Fields as usize].level, 10); // too strict?
    assert!(game.state.works[WorkTypes::Mill as usize].level >= 10); // too strict?

    do_test_rebirth(game);
    assert!(
        game.state.rebirth_stats.coins >= 3.0,
        "coins = {:#?}",
        game.state.rebirth_stats.coins
    );
}

#[wasm_bindgen_test]
fn test_tenth_rebirth() {
    let game = &mut Game::new();
    game.load_game(tenth_rebirth());
    run_until_dead(game);
    assert_eq!(game.state.works[WorkTypes::Fields as usize].level, 10); // too strict?
    assert!(game.state.works[WorkTypes::Mill as usize].level >= 10); // too strict?

    let correct_tomb_purchased = game.state.tombs[TombTypes::BurialPit as usize].is_purchased;
    assert!(
        correct_tomb_purchased,
        "items = {:#?} \n tombs = {:#?}",
        game.state.items, game.state.tombs
    );
    do_test_rebirth(game);
}
