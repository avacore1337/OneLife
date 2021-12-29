// #![cfg(target_arch = "wasm32")]

use one_life::game::Game;
// use one_life::input::options::AutoSettingTypes;
use one_life::input::work::WorkTypes;
use one_life::presets::second_rebirth;

use one_life::util::{do_test_rebirth, run_until_dead, set_full_auto};
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_first_rebirth() {
    let game = &mut Game::new();
    set_full_auto(&mut game.meta_data.options);
    run_until_dead(game);
    assert_eq!(game.state.works[WorkTypes::GalleyRower as usize].level, 10);

    do_test_rebirth(game);
    assert_eq!(game.state.rebirth_stats.coins, 0.0);
}

#[wasm_bindgen_test]
fn test_second_rebirth() {
    let game = &mut Game::new();
    game.load_game(second_rebirth(&game.world));
    run_until_dead(game);
    assert_eq!(game.state.works[WorkTypes::Fields as usize].level, 10); // too strict?
    assert!(game.state.works[WorkTypes::Mill as usize].level >= 10); // too strict?

    do_test_rebirth(game);
    assert_eq!(game.state.rebirth_stats.coins, 2.0);
    // assert_eq!(
    //     format!("{:#?}", game.state.rebirth_stats.max_job_levels),
    //     ""
    // );
}
