// #![cfg(target_arch = "wasm32")]

use one_life::game::Game;
// use one_life::input::options::AutoSettingTypes;
use one_life::input::work::WorkTypes;
use one_life::presets::third_rebirth;

use one_life::util::{do_test_rebirth, run_until_dead};
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_second_rebirth() {
    let game = &mut Game::new();
    game.load_game(third_rebirth(&game.world));
    run_until_dead(game);
    assert_eq!(game.state.works[WorkTypes::Fields as usize].level, 10); // too strict?
    assert!(game.state.works[WorkTypes::Mill as usize].level >= 10); // too strict?

    do_test_rebirth(game);
    assert!(game.state.rebirth_stats.coins > 4.0);
    // assert_eq!(
    //     format!("{:#?}", game.state.rebirth_stats.max_job_levels),
    //     ""
    // );
}
