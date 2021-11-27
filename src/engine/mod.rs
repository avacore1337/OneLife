pub mod intermediate_state;
pub mod value_keys;

use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::info::check_for_tutorial_step;
use crate::input::activity::ActivityTypes;
use crate::input::housing::HousingTypes;
use crate::input::stat::StatTypes;
use crate::input::work::WorkTypes;
use crate::state::state_container::StateContainer;
use crate::state::stats::Stat;
use crate::state::work::Work as StateWork;
use crate::world_content::activity::{should_unlock_activity, translate_activity};
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
    let _old_state = game.state.clone(); //TODO use for delta?

    // Apply all modifiers to intermediate
    apply_housing(game);
    apply_items(game);
    apply_work(game);
    apply_activities(game);

    // Get the gains
    do_work(game.input.work, &mut game.state);
    gain_work_xp(game.input.work as usize, &mut game.state);
    gain_stat_xp(game);

    // update frontend read values
    update_unlocks(game);
    update_life_stats(game);
    check_for_tutorial_step(game);
}

fn update_life_stats(game: &mut Game) {
    let life_stats = &mut game.state.life_stats;
    life_stats.happiness = game.intermediate_state.get_multiplier(KeyValues::Happiness);
    life_stats.health += 0.000_002 * game.intermediate_state.get_value(KeyValues::Health);

    // Base gamespeed is that one life should take 30min for 0.0 health, the game runs in 30 ticks/second
    // Days/tick = total_days / (ticks in 30 min)
    // 52*365/(30*60*30) = 0.351
    life_stats.age += 0.35 * game.state.rebirth_stats.time_factor;
    life_stats.lifespan = crate::BASE_LIFESPAN * (1.0 + life_stats.health);

    let should_die = life_stats.age + life_stats.health >= life_stats.lifespan;
    if should_die {
        life_stats.dead = true;
        character_death_update(game);
    }
}

fn update_unlocks(game: &mut Game) {
    for work in WorkTypes::iter() {
        game.state.works[work as usize].is_unlocked = should_unlock_work(work, game);
    }
    for work in WorkTypes::iter() {
        game.state.works[work as usize].is_visible = should_be_visable_work(work, game);
    }
    for activity in ActivityTypes::iter() {
        game.state.activity[activity as usize].is_unlocked = should_unlock_activity(activity, game);
    }
}

fn calculate_intermediate_state(_game: &Game) -> IntermediateState {
    IntermediateState::new()
}

fn character_death_update(game: &mut Game) {
    for (index, work) in game.state.works.iter().enumerate() {
        game.state.rebirth_stats.max_job_levels[index] =
            std::cmp::max(game.state.rebirth_stats.max_job_levels[index], work.level);
    }
    game.state.rebirth_stats.coins += 2.0;
}

fn apply_housing(game: &mut Game) {
    let mut housing = translate_housing(game.input.housing);
    if housing.upkeep > game.state.items.money {
        housing = translate_housing(HousingTypes::StoneFloor);
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

fn apply_work(game: &mut Game) {
    let work = translate_work(game.input.work);
    game.intermediate_state.get_gains(&work);
}

fn apply_activities(game: &mut Game) {
    let activity = translate_activity(game.input.activity);
    game.intermediate_state.get_gains(&activity);
}

fn gain_work_xp(input_work: usize, state: &mut StateContainer) {
    let work: &mut StateWork = &mut state.works[input_work];
    work.next_level_progress += 0.3
        * state.life_stats.happiness
        * (1.0 + f64::sqrt(state.rebirth_stats.max_job_levels[input_work] as f64));
    let mut next_level_xp_needed = calculate_work_next_level_xp_neeeded(work);
    while work.next_level_progress > next_level_xp_needed {
        work.level += 1;
        work.next_level_progress -= next_level_xp_needed;
        next_level_xp_needed = calculate_work_next_level_xp_neeeded(work);
    }
    work.next_level_required = next_level_xp_needed;
    work.next_level_percentage = (work.next_level_progress * 100.0) / next_level_xp_needed;
}

fn gain_stat_xp(game: &mut Game) {
    let int_level = game.state.base_stats[StatTypes::Int as usize].level;
    let stat_xp_multiplier = 1.0 + (int_level as f64 / 10.0);
    for stat_type in StatTypes::iter() {
        let stat_xp = game.intermediate_state.get_value(stat_type.into());
        let stat: &mut Stat = &mut game.state.base_stats[stat_type as usize];
        stat.next_level_progress += stat_xp * stat_xp_multiplier;
        let mut next_level_xp_needed = calculate_stat_next_level_xp_neeeded(stat);
        while stat.next_level_progress > next_level_xp_needed {
            stat.level += 1.0;
            stat.next_level_progress -= next_level_xp_needed;
            next_level_xp_needed = calculate_stat_next_level_xp_neeeded(stat);
        }
        stat.next_level_required = next_level_xp_needed;
        stat.next_level_percentage = (stat.next_level_progress * 100.0) / next_level_xp_needed;
    }
}

fn calculate_stat_next_level_xp_neeeded(stat: &mut Stat) -> f64 {
    (100.0 + (4.0 * stat.level * stat.level)) as f64
}

fn calculate_work_next_level_xp_neeeded(work: &mut StateWork) -> f64 {
    (100 + (4 * work.level * work.level)) as f64
}

fn do_work(input_work: WorkTypes, state: &mut StateContainer) {
    let work = translate_work(input_work);
    let main_stat_level = state.base_stats[work.main_stat as usize].level;
    let work_state = state.works[input_work as usize];
    let level_multiplier: f64 = 1.0 + (work_state.level as f64 / 10.0);
    let stat_multiplier: f64 = 1.0 + (main_stat_level as f64 / 10.0);
    state.items.money += work.money * level_multiplier * stat_multiplier / 30.0;
}
