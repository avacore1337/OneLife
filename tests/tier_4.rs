// #![cfg(target_arch = "wasm32")]

use one_life::game::Game;
use one_life::input::boost_item::BoostItemTypes;
use one_life::input::tomb::TombTypes;
// use one_life::input::work::WorkTypes;
use one_life::presets::rebirth_36;

use one_life::util::run_until_dead;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_rebirth_36() {
    let game = &mut Game::new();
    game.load_game(rebirth_36());
    run_until_dead(game);

    let money = game.state.items.money;
    // assert!(
    //     game.state.works[WorkTypes::Hypaspist as usize].level >= 50,
    //     "Hypaspist level: {}",
    //     game.state.works[WorkTypes::Hypaspist as usize].level
    // );
    assert!(
        game.state.boost_items[BoostItemTypes::Burial3 as usize].is_purchased,
        "No burial 3 purchased, money: {}",
        money
    );
    assert!(
        game.state.tombs[TombTypes::FamilyGrave as usize].is_purchased,
        "No FamilyGrave purchased. Money: {}",
        money
    );
    assert!(money < 15.0e6, "Too much money money: {}", money);
}
