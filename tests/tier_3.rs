// #![cfg(target_arch = "wasm32")]

use one_life::game::Game;
use one_life::input::boost_item::BoostItemTypes;
use one_life::input::tomb::TombTypes;
// use one_life::input::options::AutoSettingTypes;
use one_life::input::work::WorkTypes;
use one_life::presets::{rebirth_24, rebirth_35};

use one_life::util::run_until_dead;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_rebirth_24() {
    let game = &mut Game::new();
    game.load_game(rebirth_24());
    run_until_dead(game);
    assert!(
        game.state.works[WorkTypes::Hypaspist as usize].level >= 10,
        "Hypaspist level: {}",
        game.state.works[WorkTypes::Hypaspist as usize].level
    );
    let money = game.state.items.money;
    let burial3_cost = game.world.boost_items[BoostItemTypes::Burial3 as usize].purchasing_cost;
    assert!(money > burial3_cost, "Can't afford Burial3: {}", money);
    assert!(money < 7.0e6, "too much money: {}", money);
}

#[wasm_bindgen_test]
fn test_rebirth_35() {
    let game = &mut Game::new();
    game.load_game(rebirth_35());
    run_until_dead(game);

    let money = game.state.items.money;
    assert!(
        game.state.works[WorkTypes::Hypaspist as usize].level >= 40,
        "Hypaspist level: {}",
        game.state.works[WorkTypes::Hypaspist as usize].level
    );
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
    assert!(money < 10.0e6, "Too much money money: {}", money);
}
