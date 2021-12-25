// #![cfg(target_arch = "wasm32")]

use one_life::engine::{character_death_update, engine_run};
use one_life::game::Game;
use one_life::input::tomb::TombTypes;
use one_life::input::work::WorkTypes;
use one_life::input::Input;

use one_life::state::state_container::rebirth;
use wasm_bindgen_test::wasm_bindgen_test;

fn set_full_auto(game: &mut Game) {
    game.meta_data.options.auto_work = true;
    game.meta_data.options.auto_living = true;
    game.meta_data.options.auto_buy_item = true;
    game.meta_data.options.auto_buy_tomb = true;
}

fn run_until_dead(game: &mut Game) {
    while !game.state.life_stats.is_dying {
        engine_run(game);
    }
    character_death_update(game);
    assert!(game.state.life_stats.dead);
}

fn do_rebirth(game: &mut Game) {
    game.state.rebirth_stats.rebirth_count += 1;
    game.state = rebirth(&game.world, game.state.rebirth_stats.clone());
    game.input = Input::new(&game.state, &game.world);
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
fn test_first_rebirth() {
    let mut game = Game::new();
    set_full_auto(&mut game);
    run_until_dead(&mut game);
    assert_eq!(game.state.works[WorkTypes::GalleyRower as usize].level, 10);
    do_rebirth(&mut game);
    assert_eq!(game.state.rebirth_stats.coins, 0.0);

    set_full_auto(&mut game);
    run_until_dead(&mut game);
    assert_eq!(game.state.works[WorkTypes::Mill as usize].level, 10); // too strict?
    do_rebirth(&mut game);
    assert_eq!(game.state.rebirth_stats.coins, 2.0);
    // Won't work because all money is spent on items, stop spending at good tick?
    // assert_eq!(
    //     format!("{:#?}", game.state.rebirth_stats.max_job_levels),
    //     ""
    // );
}
