pub mod intermediate_state;
pub mod value_keys;

use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::input::housing::Housing as InputHousing;
use crate::input::work::Work as InputWork;
use crate::state::state_container::StateContainer;
use crate::state::work::Work as StateWork;
use crate::world_content::boost_item::translate_boost_item;
use crate::world_content::housing::translate_housing;
use crate::world_content::work::{should_be_visable_work, should_unlock_work, translate_work};
use intermediate_state::IntermediateState;
use strum::IntoEnumIterator;

pub fn engine_run(game: &mut Game) {
    if game.state.life_stats.dead {
        return;
    }
    game.intermediate_state = calculate_intermediate_state(game);
    let _old_state = game.state.clone(); //TODO use?
    apply_housing(game);
    apply_items(game);
    do_work(game.input.work, &mut game.state);
    gain_work_xp(game.input.work as usize, &mut game.state);
    update_unlocks(game);
    // Base gamespeed is that one life should take 30min, the game runs in 30 ticks/second
    // Days/tick = total_days / (ticks in 30 min)
    // 52*365/(30*60*30) = 0.351
    game.state.life_stats.age += 0.35 * game.state.rebirth_stats.time_factor;

    game.state.life_stats.happiness = game.intermediate_state.get_value(KeyValues::Happiness);
    if character_should_die(game) {
        game.state.life_stats.dead = true;
        character_death_update(game);
    }
}

fn update_unlocks(game: &mut Game) {
    for input_work in InputWork::iter() {
        game.state.works[input_work as usize].is_unlocked = should_unlock_work(input_work, game);
    }
    for input_work in InputWork::iter() {
        game.state.works[input_work as usize].is_visible = should_be_visable_work(input_work, game);
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
    for (index, work) in game.state.works.iter().enumerate() {
        game.state.rebirth_stats.max_job_levels[index] = std::cmp::max(
            game.state.rebirth_stats.max_job_levels[index],
            work.current_level,
        );
    }
    game.state.rebirth_stats.coins += 2.0;
}

fn apply_housing(game: &mut Game) {
    let mut housing = translate_housing(game.input.housing);
    if housing.upkeep > game.state.items.money {
        housing = translate_housing(InputHousing::StoneFloor);
        // TODO signal to frontend that you are out of cash?
    }
    game.state.items.money -= housing.upkeep;
    game.intermediate_state.get_gains(&housing);
}

fn apply_items(game: &mut Game) {
    for item in game.state.items.boost_items {
        if item.is_purchased {
            let boost_item = translate_boost_item(item.name);
            game.intermediate_state.get_gains(&boost_item);
            //
        }
    }
}

fn gain_work_xp(input_work: usize, state: &mut StateContainer) {
    let work: &mut StateWork = &mut state.works[input_work];
    work.next_level_progress += 0.3
        * state.life_stats.happiness
        * (1.0 + f64::sqrt(state.rebirth_stats.max_job_levels[input_work] as f64));
    let mut next_level_xp_needed = calculate_next_level_xp_neeeded(work);
    while work.next_level_progress > next_level_xp_needed {
        work.current_level += 1;
        work.next_level_progress -= next_level_xp_needed;
        next_level_xp_needed = calculate_next_level_xp_neeeded(work);
    }
    work.next_level_percentage = (work.next_level_progress * 100.0) / next_level_xp_needed;
}

fn calculate_next_level_xp_neeeded(work: &mut StateWork) -> f64 {
    (100 + (4 * work.current_level * work.current_level)) as f64
}

fn do_work(input_work: InputWork, state: &mut StateContainer) {
    let work = translate_work(input_work);
    let work_state = state.works[input_work as usize];
    let multiplier: f64 = 1.0 + (work_state.current_level as f64 / 10.0);
    state.items.money += work.money * multiplier;
}
