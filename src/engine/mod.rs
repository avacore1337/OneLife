pub mod intermediate_state;

use crate::game::Game;
use crate::input::work::Work as InputWork;
use crate::state::state_container::StateContainer;
use crate::state::work::Work as StateWork;
use crate::world_content::work::translate_work;
use intermediate_state::IntermediateState;

pub fn engine_run(game: &mut Game, time_delta: f64) {
    let modified_time_delta = time_delta * game.meta_data.game_speed;
    if game.state.life_stats.dead {
        return;
    }
    game.intermediate_state = calculate_intermediate_state(game);
    let mut new_state = game.state.clone();
    do_work(game.input.work, &mut new_state);
    gain_work_xp(game.input.work, &mut new_state);
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

fn gain_work_xp(input_work: InputWork, state: &mut StateContainer) {
    let work: &mut StateWork = &mut state.works[input_work as usize];
    work.next_level_progress += 10.0 * state.life_stats.happiness;
    let mut next_level_xp_needed = calculate_next_level_xp_neeeded(work);
    while work.next_level_progress > next_level_xp_needed {
        work.current_level += 1;
        work.next_level_progress -= next_level_xp_needed;
        next_level_xp_needed = calculate_next_level_xp_neeeded(work);
    }
    work.next_level_percentage = (work.next_level_progress * 100.0) / next_level_xp_needed;
}

fn calculate_next_level_xp_neeeded(work: &mut StateWork) -> f64 {
    (100 * work.current_level) as f64
}

fn do_work(input_work: InputWork, state: &mut StateContainer) {
    let work = translate_work(input_work);
    let work_state = state.works[input_work as usize];
    let multiplier: f64 = 1.0 + (work_state.current_level as f64 / 10.0);
    state.items.money += work.money * multiplier;
}
