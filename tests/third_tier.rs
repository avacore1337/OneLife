// #![cfg(target_arch = "wasm32")]

use one_life::game::Game;
use one_life::input::boost_item::BoostItemTypes;
use one_life::input::tomb::TombTypes;
// use one_life::input::options::AutoSettingTypes;
use one_life::input::work::WorkTypes;
use one_life::presets::{rebirth_15, rebirth_22};

use one_life::util::run_until_dead;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_fifteenth_rebirth() {
    let game = &mut Game::new();
    game.load_game(rebirth_15());
    run_until_dead(game);
    assert_eq!(game.state.works[WorkTypes::Peltasts as usize].level, 10); // too strict?
}

#[wasm_bindgen_test]
fn test_rebirth_22() {
    let game = &mut Game::new();
    game.load_game(rebirth_22());
    run_until_dead(game);
    assert!(
        game.state.works[WorkTypes::FootCompanion as usize].level >= 18,
        "FootCompanion level: {}",
        game.state.works[WorkTypes::FootCompanion as usize].level
    );
    assert!(
        game.state.boost_items[BoostItemTypes::Burial2 as usize].is_purchased,
        "money: {}",
        game.state.items.money,
    );
    assert!(
        game.state.tombs[TombTypes::Grave as usize].is_purchased,
        "money: {}",
        game.state.items.money,
    );
    assert!(
        game.state.items.money < 1_000_000.0,
        "money: {}",
        game.state.items.money,
    );
}
