pub mod intermediate_state;

use crate::game::Game;
use crate::input::work::Work;
use crate::state::state_container::StateContainer;
use crate::world_content::work::translate_work;
use intermediate_state::IntermediateState;

pub fn engine_run(game: &mut Game, time_delta: f64) {
    let modified_time_delta = time_delta * game.meta_data.game_speed;
    if game.state.life_stats.dead {
        return;
    }
    game.intermediate_state = calculate_intermediate_state(game);
    let mut new_state = game.state.clone();
    do_work(&game.input.work, &mut new_state);
    new_state.life_stats.age += modified_time_delta * new_state.rebirth_stats.time_factor;

    game.state.life_stats.happiness = game
        .intermediate_state
        .value_gains
        .get("happiness")
        .map(|value_gains| value_gains.calculate_value())
        .unwrap_or(1.0);

    game.state = new_state;
    if character_should_die(game) {
        game.state.life_stats.dead = true;
        character_death_update(game);
    }
}

fn calculate_intermediate_state(_game: &Game) -> IntermediateState {
    IntermediateState::new()
}

fn character_should_die(game: &Game) -> bool {
    let life_stats = &game.state.life_stats;
    life_stats.age + life_stats.health >= life_stats.lifespan
}

fn character_death_update(game: &mut Game) {
    game.state.rebirth_stats.coins += 2.0;
}

fn do_work(input_work: &Work, state: &mut StateContainer) {
    let work = translate_work(input_work);
    state.items.money += work.money;
}
